use hdf5_rs::{
    cchar_to_string,
    error::H5Error,
    h5sys::CStr,
    types::{
        AttrOpener, AttributeFillable, DataSpaceOwner, DatasetOwner, File,
        FileOpenAccess, Group, GroupOpener,
    },
};
use spike_rs::error::SpikeError;

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

pub enum PhaseType {
    RawData,
    Digital,
    STGEvents,
    Unknown,
}

pub struct AnalogStream {
    pub info_channels: Vec<info_channel::CInfoChannel>,
    pub labels: Vec<String>,
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

        let mut info_channels =
            vec![info_channel::CInfoChannel::default(); ic_dims[0]];

        info_channel.fill_memory(
            info_channel::info_channel_type(),
            &mut info_channels,
        )?;

        let mut labels = vec![];

        for (i, ic) in info_channels.iter().enumerate() {
            if i == 0 {
                sampling_frequency = ic.tick * 100;
            } else {
                if ic.tick * 100 != sampling_frequency {
                    return Err(SpikeH5Error::FileError(
                            FileErrorType::AnalogChannelsHaveDifferentSamplingFrequencies));
                }
            }
            labels.push(cchar_to_string!(ic.label));
        }

        let ret = Self { info_channels, labels };

        if label.contains("Electrode Raw Data") {
            Ok((ret, PhaseType::RawData, sampling_frequency, duration as i64))
        } else if label.contains("Digital Data") {
            Ok((ret, PhaseType::Digital, sampling_frequency, duration as i64))
        } else {
            Ok((ret, PhaseType::Unknown, sampling_frequency, duration as i64))
        }
    }
}

pub struct EventStream {}

impl EventStream {
    pub fn parse(group: Group) -> Result<Self, SpikeH5Error> {
        todo!()
    }
}

pub struct PhaseH5 {
    pub timestamp: u64,
    pub date: String,
    pub raw_data: AnalogStream,
    pub digital: Option<AnalogStream>,
    pub events: Option<EventStream>,
    pub sampling_frequency: f32,
    pub duration: usize,
}

impl PhaseH5 {
    pub fn open(filepath: &str) -> Result<Self, SpikeH5Error> {
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
                if label.contains("STG Events") {}
                println!("EventStream: {}", label);
                println!("{:?}", event.list_groups());
                println!("------------------------------");
                println!("{:?}", event.list_datasets());
            }
        }

        Ok(Self {
            timestamp,
            date,
            raw_data: raw_data.unwrap(),
            digital,
            events,
            sampling_frequency: raw_data_sf as f32,
            duration: raw_data_duration as usize,
        })
    }
}
