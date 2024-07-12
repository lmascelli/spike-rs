use hdf5_rs::{
    cchar_to_string,
    error::H5Error,
    h5sys::CStr,
    types::{
        AttrOpener, AttributeFillable, DataSet, DataSetWriter, DataSpaceOwner,
        DatasetFillable, DatasetOwner, File, FileOpenAccess, Group,
        GroupOpener,
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
        let mut trains = vec![];
        for label in group.list_groups() {
            let channel_group = group.open_group(&label)?;
            trains.push((
                label.clone(),
                channel_group.get_dataset("sample")?,
                channel_group.get_dataset("value")?,
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
        }

        Ok(Self {
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
                    if data.len() == (actual_end-actual_start) {
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
    ) -> Result<(Vec<f32>, Vec<usize>), SpikeError> {
        if self.peak_train.is_some() {
            if let Some(train) = self
                .peak_train
                .as_ref()
                .unwrap()
                .trains
                .iter()
                .filter(|v| v.0 == channel)
                .next()
            {
                if let Ok(times) = i64::from_dataset(&train.1, None) {
                    if let Ok(values) = f32::from_dataset(&train.2, None) {
                        todo!()
                    } else {
                        Err(SpikeError::OperationFailed)
                    }
                } else {
                    Err(SpikeError::OperationFailed)
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
        data: (Vec<f32>, Vec<usize>),
    ) -> Result<(), SpikeError> {
        // get all values before start
        // get all values after end
        // join the values with data
        // delete the old dataspace
        // create a new one
        todo!()
    }
}
