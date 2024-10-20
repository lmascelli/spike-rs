use hdf5_rs::{
    cchar_to_string,
    error::H5Error,
    h5sys::CStr,
    types::{
        AttrOpener, AttributeFillable, CreateDataSetOptions,
        CreateGroupOptions, DataSet, DataSetWriter, DataSpace, DataSpaceOwner,
        DataSpaceType, DatasetFillable, DatasetOwner, File, FileOpenAccess,
        Group, GroupOpener, IntoDataType,
    },
};
use info_channel::CInfoChannel;
use spike_rs::error::SpikeError;
use spike_rs::types::PhaseHandler;

mod info_channel;

#[derive(Debug)]
pub enum FileErrorType {
    UnknownAnalogType,
    PhaseHasNoRawData,
    PhaseHasMultipleRawData,
    PhaseHasMultipleDigital,
    DigitalSamplesDoesNotMatchRawDataSamples,
    ChannelDataDimDoesntMatchInfoChannelDims,
    ChannelDataIsNotAMatrix,
    RawDatasHaveDifferentSamples,
    InfoChannelDataSetNotAVector,
    AnalogChannelsHaveDifferentSamplingFrequencies,
    RawDataAndDigitalHaveDifferentSamplingFrequencies,
    SystemTimeError,
    MultipleEventStreams,
}

#[derive(Debug)]
pub enum SpikeH5Error {
    SpikeError(SpikeError),
    H5LibError(H5Error),
    IOError(std::io::Error),
    FileError(FileErrorType),
}

impl From<H5Error> for SpikeH5Error {
    fn from(value: H5Error) -> Self {
        SpikeH5Error::H5LibError(value)
    }
}

impl From<SpikeError> for SpikeH5Error {
    fn from(value: SpikeError) -> Self {
        SpikeH5Error::SpikeError(value)
    }
}

impl From<std::io::Error> for SpikeH5Error {
    fn from(value: std::io::Error) -> Self {
        SpikeH5Error::IOError(value)
    }
}

impl From<std::time::SystemTimeError> for SpikeH5Error {
    fn from(_value: std::time::SystemTimeError) -> Self {
        SpikeH5Error::FileError(FileErrorType::SystemTimeError)
    }
}

#[derive(Debug)]
pub struct PeakTrain {
    pub trains: Vec<(String, DataSet, DataSet)>,
}

impl PeakTrain {
    pub fn parse(group: Group) -> Result<Self, SpikeH5Error> {
        // peak trains should be contained in a separated group for each channel
        // each of which should contain a `sample` and a `value` dataset
        let mut trains = vec![];
        for label in group.list_groups() {
            let channel_group = group.open_group(&label)?;
            trains.push((
                label.clone(),
                channel_group.get_dataset("samples")?,
                channel_group.get_dataset("values")?,
            ));
        }
        Ok(Self { trains })
    }
}

pub enum PhaseType {
    RawData,
    Digital,
    STGEvents,
    Unknown,
}

pub struct InfoChannelData {
    label: String,
    row: usize,
    conversion_factor: f32,
    offset: f32,
}

impl InfoChannelData {
    fn from_info_channel(ic: CInfoChannel) -> Self {
        Self {
            label: cchar_to_string!(ic.label),
            row: ic.row_index as usize,
            conversion_factor: ic.conversion_factor as f32
                * 10f32.powf(ic.exponent as f32),
            offset: ic.ad_zero as f32,
        }
    }
}

pub struct AnalogStream {
    pub label: String,
    pub info_channels: Vec<InfoChannelData>,
    pub channel_data: DataSet,
    pub labels: Vec<String>,
}

impl std::fmt::Debug for AnalogStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "AnalogStream {}.\nN° channels: {}",
            self.label,
            self.info_channels.len()
        )?;
        Ok(())
    }
}

impl AnalogStream {
    /// Parse and return a "Stream_X" group determining its [PhaseType],
    /// the `sampling_frequency`, and the duration.
    ///
    /// The return value means (Self, PhaseType, sampling_frequency, duration)
    pub fn parse(
        group: Group,
    ) -> Result<(Self, PhaseType, i64, i64), SpikeH5Error> {
        let info_channel = group.get_dataset("InfoChannel")?;
        let info_channel_dataspace = info_channel.get_space()?;
        let ic_dims = info_channel_dataspace.get_dims();
        let channel_data = group.get_dataset("ChannelData")?;
        let channel_data_dataspace = channel_data.get_dataspace()?;
        let dims = channel_data_dataspace.get_dims();
        let duration = dims[1];
        let label = String::from_attribute(&group.open_attr("Label")?)?;

        let mut sampling_frequency = 0;

        let mut c_info_channels =
            vec![info_channel::CInfoChannel::default(); ic_dims[0] as usize];

        let mut info_channels = vec![];

        info_channel.fill_memory(
            info_channel::info_channel_type(),
            &mut c_info_channels,
        )?;

        let mut labels = vec![];

        for (i, ic) in c_info_channels.iter().enumerate() {
            if i == 0 {
                sampling_frequency = ic.tick * 100;
            } else if ic.tick * 100 != sampling_frequency {
                return Err(SpikeH5Error::FileError(
                            FileErrorType::AnalogChannelsHaveDifferentSamplingFrequencies));
            }
            labels.push(cchar_to_string!(ic.label));
            info_channels.push(InfoChannelData::from_info_channel(*ic));
        }

        let ret = Self { label, info_channels, channel_data, labels };

        if ret.label.contains("Electrode Raw Data") {
            Ok((ret, PhaseType::RawData, sampling_frequency, duration as i64))
        } else if ret.label.contains("Digital Data") {
            Ok((ret, PhaseType::Digital, sampling_frequency, duration as i64))
        } else {
            Ok((ret, PhaseType::Unknown, sampling_frequency, duration as i64))
        }
    }
}

pub struct EventStream {
    pub label: String,
    pub channel: DataSet,
}

impl std::fmt::Debug for EventStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "EventStream: {}", self.label)?;
        writeln!(f, "{}", self.channel.get_path())?;
        Ok(())
    }
}

impl EventStream {
    pub fn parse(group: Group) -> Result<Vec<Self>, SpikeH5Error> {
        let mut events = vec![];

        for ds_name in group.list_datasets() {
            if ds_name.starts_with("EventEntity_") {
                events.push(Self {
                    channel: group.get_dataset(&ds_name)?,
                    label: ds_name,
                });
            }
        }
        Ok(events)
    }
}

#[derive(Debug)]
pub struct PhaseH5 {
    pub file: File,
    pub timestamp: u64,
    pub date: String,
    pub raw_data: AnalogStream,
    pub digital: Option<AnalogStream>,
    pub events: Vec<EventStream>,
    pub peak_train: Option<PeakTrain>,
    pub sampling_frequency: f32,
    pub datalen: usize,
}

impl PhaseH5 {
    pub fn open(filepath: &str) -> Result<Self, SpikeH5Error> {
        hdf5_rs::init()?;

        // read the timestamp of the file
        // for future use in ordering the phase without parsing the name of the file
        let timestamp;
        {
            let file = std::fs::File::open(filepath)?;
            timestamp = file
                .metadata()?
                .created()?
                .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                .as_secs();
        }

        let file = File::open(filepath, FileOpenAccess::ReadWrite)?;

        // retrive the date of the recording
        let date;
        {
            let data_group = file.open_group("/Data")?;
            let data_attribute = data_group.open_attr("Date")?;
            date = String::from_attribute(&data_attribute)?;
        }

        let mut raw_data = None;
        let mut raw_data_sf = 0;
        let mut raw_data_duration = 0;
        let mut digital = None;
        let mut digital_sf = 0;
        let mut digital_duration = 0;
        let mut events = None;
        let mut peak_train = None;

        // parse the analog streams to get the raw data and the digital data (if available)
        let analog_streams =
            file.open_group("/Data/Recording_0/AnalogStream")?;
        for group in analog_streams.list_groups() {
            let (analog, analog_type, sampling_frequency, duration) =
                AnalogStream::parse(analog_streams.open_group(&group)?)?;
            match analog_type {
                PhaseType::Digital => {
                    if digital.is_none() {
                        digital.replace(analog);
                        digital_sf = sampling_frequency;
                        digital_duration = duration;
                    } else {
                        return Err(SpikeH5Error::FileError(
                            FileErrorType::PhaseHasMultipleDigital,
                        ));
                    }
                }
                PhaseType::RawData => {
                    if raw_data.is_none() {
                        raw_data.replace(analog);
                        raw_data_sf = sampling_frequency;
                        raw_data_duration = duration;
                    } else {
                        return Err(SpikeH5Error::FileError(
                            FileErrorType::PhaseHasMultipleRawData,
                        ));
                    }
                }
                _ => {
                    return Err(SpikeH5Error::FileError(
                        FileErrorType::UnknownAnalogType,
                    ));
                }
            }
        }

        if raw_data.is_none() {
            return Err(SpikeH5Error::FileError(
                FileErrorType::PhaseHasNoRawData,
            ));
        }
        if digital.is_some() {
            if raw_data_sf != digital_sf {
                return Err(SpikeH5Error::FileError(
                FileErrorType::RawDataAndDigitalHaveDifferentSamplingFrequencies,
            ));
            }

            if raw_data_duration != digital_duration {
                return Err(SpikeH5Error::FileError(
                    FileErrorType::DigitalSamplesDoesNotMatchRawDataSamples,
                ));
            }
        }

        // parse the event_streams (if any)
        if let Ok(event_streams) =
            file.open_group("/Data/Recording_0/EventStream")
        {
            for group_name in event_streams.list_groups() {
                let event = event_streams.open_group(&group_name)?;
                let label = String::from_attribute(&event.open_attr("Label")?)?;
                if label.contains("STG Events") {
                    if events.is_none() {
                        events.replace(EventStream::parse(event)?);
                    } else {
                        return Err(SpikeH5Error::FileError(
                            FileErrorType::MultipleEventStreams,
                        ));
                    }
                }
            }
        }

        // parse the peak_train stream (if any)
        if let Ok(peak_train_group) =
            file.open_group("/Data/Recording_0/Peak_Train")
        {
            peak_train.replace(PeakTrain::parse(peak_train_group)?);
        } else {
            if let Err(err) = file.create_group(CreateGroupOptions {
                loc_id: file.fid,
                path: "/Data/Recording_0/Peak_Train".to_string(),
                link_creation_properties: None,
                group_creation_properties: None,
                group_access_properties: None,
            }) {
                return Err(SpikeH5Error::H5LibError(err));
            }
        }

        Ok(Self {
            file,
            timestamp,
            date,
            raw_data: raw_data.unwrap(),
            digital,
            events: events.unwrap_or(vec![]),
            peak_train,
            sampling_frequency: raw_data_sf as f32,
            datalen: raw_data_duration as usize,
        })
    }

    fn insert_peak_train(
        &mut self,
        channel: &str,
        data: (Vec<usize>, Vec<f32>),
    ) -> Result<(), SpikeError> {
        /*
         * Questa funzione deve controllare se esiste il grouppo corrispondente al canale,
         * se no crearlo. Poi deve eliminare i dataset del grouppo e sostituirli con i nuovi
         * passati. Infine collegare i dataset creati nella struttura della fase
         * */

        // Check if the group `/Peak_Train/{channel}` exists and create it if not
        let spike_group = match self
            .file
            .open_group("/Data/Recording_0/Peak_Train")
        {
            Ok(channels_group) => {
                if !channels_group.list_groups().contains(&format!("{channel}"))
                {
                    match channels_group.create_group(CreateGroupOptions {
                        loc_id: self.file.fid,
                        path: format!("/Data/Recording_0/Peak_Train/{channel}"),
                        link_creation_properties: None,
                        group_creation_properties: None,
                        group_access_properties: None,
                    }) {
                        Err(err) => {
                            eprintln!(
                                "Error: SpikeH5::insert_peak_train {:?}",
                                err
                            );
                            return Err(SpikeError::OperationFailed);
                        }
                        _ => (),
                    }
                    self.file
                        .open_group(&format!(
                            "/Data/Recording_0/Peak_Train/{channel}"
                        ))
                        .unwrap()
                } else {
                    let spike_group = self
                        .file
                        .open_group(&format!(
                            "/Data/Recording_0/Peak_Train/{channel}"
                        ))
                        .unwrap();

                    // delete the old datasets and replace the new ones
                    match spike_group.get_dataset("samples") {
                        Ok(mut samples_ds) => {
                            samples_ds.delete();
                        }
                        Err(err) => {
                            eprintln!(
                                "Error: SpikeH5::insert_peak_train {:?}",
                                err
                            );
                            return Err(SpikeError::OperationFailed);
                        }
                    }

                    match spike_group.get_dataset("values") {
                        Ok(mut values_ds) => {
                            values_ds.delete();
                        }
                        Err(err) => {
                            eprintln!(
                                "Error: SpikeH5::insert_peak_train {:?}",
                                err
                            );
                            return Err(SpikeError::OperationFailed);
                        }
                    }
                    spike_group
                }
            }
            Err(err) => {
                eprintln!("Error: SpikeH5::insert_peak_train {:?}", err);
                return Err(SpikeError::OperationFailed);
            }
        };

        // Create the new datasets
        let mut times_ds =
            match spike_group.create_dataset(CreateDataSetOptions {
                name: "samples",
                link_plist: None,
                create_plist: None,
                access_plist: None,
                dtype: match usize::into_datatype() {
                    Ok(dt) => dt,
                    Err(err) => {
                        eprintln!(
                            "Error: SpikeH5::insert_peak_train {:?}",
                            err
                        );
                        return Err(SpikeError::OperationFailed);
                    }
                },
                dspace: match DataSpace::create_dataspace(
                    DataSpaceType::Simple,
                    &[data.0.len() as u64],
                ) {
                    Ok(ds) => ds,
                    Err(err) => {
                        eprintln!(
                            "Error: SpikeH5::insert_peak_train {:?}",
                            err
                        );
                        return Err(SpikeError::OperationFailed);
                    }
                },
            }) {
                Ok(ds) => ds,
                Err(err) => {
                    eprintln!("Error: SpikeH5::insert_peak_train {:?}", err);
                    return Err(SpikeError::OperationFailed);
                }
            };

        let mut values_ds =
            match spike_group.create_dataset(CreateDataSetOptions {
                name: "values",
                link_plist: None,
                create_plist: None,
                access_plist: None,
                dtype: match f32::into_datatype() {
                    Ok(dt) => dt,
                    Err(err) => {
                        eprintln!(
                            "Error: SpikeH5::insert_peak_train {:?}",
                            err
                        );
                        return Err(SpikeError::OperationFailed);
                    }
                },
                dspace: match DataSpace::create_dataspace(
                    DataSpaceType::Simple,
                    &[data.0.len() as u64],
                ) {
                    Ok(ds) => ds,
                    Err(err) => {
                        eprintln!(
                            "Error: SpikeH5::insert_peak_train {:?}",
                            err
                        );
                        return Err(SpikeError::OperationFailed);
                    }
                },
            }) {
                Ok(ds) => ds,
                Err(err) => {
                    eprintln!("Error: SpikeH5::insert_peak_train {:?}", err);
                    return Err(SpikeError::OperationFailed);
                }
            };

        // Fill the new datasets with the data
        match data.0[..].as_ref().to_dataset(&mut times_ds, None) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("Error: SpikeH5::insert_peak_train {:?}", err);
                return Err(SpikeError::OperationFailed);
            }
        }

        match data.1[..].as_ref().to_dataset(&mut values_ds, None) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("Error: SpikeH5::insert_peak_train {:?}", err);
                return Err(SpikeError::OperationFailed);
            }
        }

        // If the vector of peak trains alredy contains the selected label
        // update it, otherwise create a new entry
        if self.peak_train.is_some() {
            let train = self
                .peak_train
                .as_mut()
                .unwrap()
                .trains
                .iter_mut()
                .find(|v| v.0 == channel);

            match train {
                Some(train) => {
                    train.1 = times_ds;
                    train.2 = values_ds;
                }
                None => {
                    // insert a new item in the vec of trains
                    self.peak_train.as_mut().unwrap().trains.push((
                        channel.to_string(),
                        times_ds,
                        values_ds,
                    ));
                }
            };
        }
        Ok(())
    }
}

impl PhaseHandler for PhaseH5 {
    fn sampling_frequency(&self) -> f32 {
        self.sampling_frequency
    }

    fn datalen(&self) -> usize {
        self.datalen
    }

    fn labels(&self) -> Vec<String> {
        self.raw_data.labels.clone()
    }

    fn raw_data(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<Vec<f32>, SpikeError> {
        if self.raw_data.labels.contains(&channel.to_string()) {
            let actual_start = start.unwrap_or(0);

            let actual_end = match end {
                Some(end) => end,
                None => self.datalen,
            };

            if actual_start < actual_end && actual_end <= self.datalen {
                let ic = self
                    .raw_data
                    .info_channels
                    .iter()
                    .find(|ic| ic.label == channel)
                    .unwrap();

                // if let Ok(cd_dataspace) = self.raw_data.channel_data.get_space()
                // {
                let start = [ic.row as u64, actual_start as u64];
                let offset = [1u64, (actual_end - actual_start) as u64];
                if let Ok(data) = i32::from_dataset_subspace(
                    &self.raw_data.channel_data,
                    &start,
                    &offset,
                    None,
                ) {
                    // check if the subspace gotten was a single row
                    // and has the same length that an actual row
                    if data.len() == (actual_end - actual_start) {
                        Ok(data
                            .iter()
                            .map(|x| {
                                *x as f32 * ic.conversion_factor + ic.offset
                            })
                            .collect())
                    } else {
                        Err(SpikeError::OperationFailed)
                    }
                } else {
                    Err(SpikeError::OperationFailed)
                }
            } else {
                Err(SpikeError::IndexOutOfRange)
            }
        } else {
            Err(SpikeError::LabelNotFound)
        }
    }

    fn set_raw_data(
        &mut self,
        channel: &str,
        start: Option<usize>,
        data: &[f32],
    ) -> Result<(), SpikeError> {
        if self.raw_data.labels.contains(&channel.to_string()) {
            let start = start.unwrap_or(0);
            if start + data.len() < self.datalen {
                let ic = self
                    .raw_data
                    .info_channels
                    .iter()
                    .find(|ic| ic.label == channel)
                    .unwrap();
                let data_c: Vec<i32> = data
                    .iter()
                    .map(|x| ((*x - ic.offset) / ic.conversion_factor) as i32)
                    .collect();
                if let Ok(()) = data_c[..].as_ref().to_dataset_row(
                    &self.raw_data.channel_data,
                    ic.row,
                    Some(start),
                    None,
                ) {
                    Ok(())
                } else {
                    Err(SpikeError::OperationFailed)
                }
            } else {
                Err(SpikeError::IndexOutOfRange)
            }
        } else {
            Err(SpikeError::LabelNotFound)
        }
    }

    fn n_digitals(&self) -> usize {
        match self.digital {
            Some(_) => 1,
            None => 0,
        }
    }

    fn digital(
        &self,
        index: usize,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<Vec<f32>, SpikeError> {
        if index == 0 && self.digital.is_some() {
            let start = start.unwrap_or(0);
            let end = end.unwrap_or(self.datalen);
            if let Ok(data) = i32::from_dataset_subspace(
                &self.digital.as_ref().unwrap().channel_data,
                &[0, start as u64],
                &[1, (end - start) as u64],
                None,
            ) {
                Ok(data.iter().map(|x| *x as f32).collect())
            } else {
                Err(SpikeError::OperationFailed)
            }
        } else {
            Err(SpikeError::IndexOutOfRange)
        }
    }

    fn set_digital(
        &mut self,
        index: usize,
        start: Option<usize>,
        data: &[f32],
    ) -> Result<(), SpikeError> {
        if index == 0 && self.digital.is_some() {
            let start = start.unwrap_or(0);
            if start + data.len() < self.datalen {
                let data: Vec<i32> = data.iter().map(|x| *x as i32).collect();
                if let Ok(()) = (&data[..]).to_dataset_row(
                    &self.digital.as_ref().unwrap().channel_data,
                    index,
                    Some(start),
                    None,
                ) {
                    Ok(())
                } else {
                    Err(SpikeError::OperationFailed)
                }
            } else {
                Err(SpikeError::IndexOutOfRange)
            }
        } else {
            Err(SpikeError::IndexOutOfRange)
        }
    }

    fn n_events(&self) -> usize {
        self.events.len()
    }

    fn events(&self, index: usize) -> Result<Vec<u64>, SpikeError> {
        if index <= self.events.len() {
            let events = &self.events[index];
            if let Ok(row) = u64::from_dataset_row(&events.channel, 0, None) {
                Ok(row)
            } else {
                Err(SpikeError::OperationFailed)
            }
        } else {
            Err(SpikeError::IndexOutOfRange)
        }
    }

    fn peak_train(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<(Vec<usize>, Vec<f32>), SpikeError> {
        if self.peak_train.is_some() {
            if let Some(train) = self
                .peak_train
                .as_ref()
                .unwrap()
                .trains
                .iter()
                .find(|v| v.0 == channel)
            {
                match usize::from_dataset(&train.1, None) {
                    Ok(times) => {
                        if let Ok(values) = f32::from_dataset(&train.2, None) {
                            if times.len() != values.len() {
                                Err(SpikeError::OperationFailed)
                            } else {
                                if times.len() == 0 {
                                    Ok((vec![], vec![]))
                                } else {
                                    if start.is_none() && end.is_none() {
                                        Ok((times, values))
                                    } else {
                                        let start = start.unwrap_or(times[0]);
                                        let end = end
                                            .unwrap_or(times[times.len() - 1]);
                                        let mut i_start = 0;
                                        let mut i_end = times.len() - 1;
                                        for (i, val) in times.iter().enumerate()
                                        {
                                            if *val >= start {
                                                i_start = i;
                                                break;
                                            }
                                        }
                                        for (i, val) in times.iter().enumerate()
                                        {
                                            if *val >= end {
                                                i_end = i;
                                                break;
                                            }
                                        }
                                        Ok((
                                            times[i_start..i_end]
                                                .iter()
                                                .map(|x| *x)
                                                .collect(),
                                            values[i_start..i_end]
                                                .iter()
                                                .map(|x| *x)
                                                .collect(),
                                        ))
                                    }
                                }
                            }
                        } else {
                            Err(SpikeError::OperationFailed)
                        }
                    }
                    Err(err) => {
                        eprintln!("Error::peak_train: {err}");
                        Err(SpikeError::OperationFailed)
                    }
                }
            } else {
                Err(SpikeError::LabelNotFound)
            }
        } else {
            Err(SpikeError::NoSpikeTrainsAvailable)
        }
    }

    fn set_peak_train(
        &mut self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
        data: (Vec<usize>, Vec<f32>),
    ) -> Result<(), SpikeError> {
        // if no spike train is saved yet just fill the dataspace with the data
        if self.peak_train.is_none() {
            return self.insert_peak_train(channel, data);
        }

        // else look for the peak of the selected channel
        match self
            .peak_train
            .as_mut()
            .unwrap()
            .trains
            .iter_mut()
            .find(|v| v.0 == channel)
        {
            None => {
                // if the channel is not saved yet just fill the dataspace with the data
                return self.insert_peak_train(channel, data);
            }
            Some(train) => {
                // if there are some data for the selected channel place the new data in the
                // correct place and replace the datasets
                let t_dataset = &mut train.1;
                let v_dataset = &mut train.2;

                let times = {
                    match usize::from_dataset(t_dataset, None) {
                        Ok(data) => data,
                        Err(_err) => {
                            return Err(SpikeError::OperationFailed);
                        }
                    }
                };

                let values = {
                    match f32::from_dataset(v_dataset, None) {
                        Ok(data) => data,
                        Err(_err) => {
                            return Err(SpikeError::OperationFailed);
                        }
                    }
                };

                if times.len() != values.len() {
                    return Err(SpikeError::OperationFailed);
                } else {
                    if times.len() == 0 {
                        // the spikes train is present but empty so just set data as the new
                        // dataset
                        return self.insert_peak_train(channel, data);
                    } else {
                        // the spikes train in present and contains data so the new data must be
                        // inserted between start and stop positions
                        let start = start.unwrap_or(times[0]);
                        let end = end.unwrap_or(times[times.len() - 1]);
                        let mut i_start = 0;
                        let mut i_end = times.len() - 1;
                        for (i, val) in times.iter().enumerate() {
                            if *val >= start {
                                i_start = i;
                                break;
                            }
                        }
                        for (i, val) in times.iter().enumerate() {
                            if *val >= end {
                                i_end = i;
                                break;
                            }
                        }

                        // get all values before start
                        let before_start_times = times[0..i_start].to_vec();
                        let before_start_values = values[0..i_start].to_vec();

                        // get all values after end
                        let after_end_times = times[i_end..].to_vec();
                        let after_end_values = values[i_end..].to_vec();

                        // join the values with data
                        let mut new_times = vec![];
                        let mut new_values = vec![];

                        new_times
                            .extend_from_slice(before_start_times.as_slice());
                        new_times.extend_from_slice(data.0.as_slice());
                        new_times.extend_from_slice(after_end_times.as_slice());

                        new_values
                            .extend_from_slice(before_start_values.as_slice());
                        new_values.extend_from_slice(data.1.as_slice());
                        new_values
                            .extend_from_slice(after_end_values.as_slice());

                        // set it as the new dataset
                        return self.insert_peak_train(
                            channel,
                            (new_times, new_values),
                        );
                    }
                };
            }
        }
    }
}
