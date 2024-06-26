use hdf5_rs::{
    cchar_to_string,
    error::Error as H5Error,
    h5sys::CStr,
    types::{
        AttrOpener, AttributeFillable, DataSpaceOwner, Dataset, DatasetOwner,
        File, FileOpenAccess, Group, GroupOpener,
    },
};
use spike_rs::{error::SpikeError, types::PhaseHandler};

mod info_channel;

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
}

pub enum PhaseH5Error {
    SpikeError(SpikeError),
    H5LibError(H5Error),
    FileError(FileErrorType),
}

impl From<H5Error> for PhaseH5Error {
    fn from(value: H5Error) -> Self {
        PhaseH5Error::H5LibError(value)
    }
}

impl From<SpikeError> for PhaseH5Error {
    fn from(value: SpikeError) -> Self {
        PhaseH5Error::SpikeError(value)
    }
}

struct AnalogStream {
    channel_data: Dataset,
    info_channels: Vec<info_channel::CInfoChannel>,
    sampling_frequency: f32,
    datalen: usize,
}

impl AnalogStream {
    fn new(group: Group) -> Result<(Self, bool), PhaseH5Error> {
        let label = String::from_attribute(&group.open_attr("Label")?)?;

        let info_channel_ds = group.get_dataset("InfoChannel")?;
        let dims = info_channel_ds.get_dataspace()?.get_dims();
        if dims.len() != 1 {
            return Err(PhaseH5Error::FileError(
                FileErrorType::InfoChannelDataSetNotAVector,
            ));
        }
        let mut info_channels =
            vec![info_channel::CInfoChannel::default(); dims[0]];
        info_channel_ds.fill_memory(
            info_channel::info_channel_type(),
            &mut info_channels,
        )?;
        let mut sampling_frequency = None;
        for ic in &info_channels {
            match sampling_frequency {
                Some(sf) => {
                    if sf != ic.tick as f32 {
                        return Err(PhaseH5Error::FileError(
                                FileErrorType::AnalogChannelsHaveDifferentSamplingFrequencies));
                    }
                }
                None => {
                    sampling_frequency = Some(ic.tick as f32);
                }
            }
        }

        let channel_data = group.get_dataset("ChannelData")?;
        let channel_data_dataspace = channel_data.get_space()?;
        let dims = channel_data_dataspace.get_dims();

        if dims.len() != 2 {
            return Err(PhaseH5Error::FileError(
                FileErrorType::ChannelDataIsNotAMatrix,
            ));
        } else {
            if dims[0] != info_channels.len() {
                return Err(PhaseH5Error::FileError(
                    FileErrorType::ChannelDataDimDoesntMatchInfoChannelDims,
                ));
            }
        }

        let datalen = dims[1];

        let ret = Self {
            channel_data,
            info_channels,
            datalen,
            sampling_frequency: sampling_frequency.unwrap() * 100.,
        };

        if label.contains("Electrode Raw Data") {
            Ok((ret, true))
        } else if label.contains("Digital Data") {
            Ok((ret, false))
        } else {
            Err(PhaseH5Error::FileError(FileErrorType::UnknownAnalogType))
        }
    }
}

struct EventStream {
    group: Group,
}

impl EventStream {
    fn new(group: Group) -> Result<Self, PhaseH5Error> {
        Ok(Self { group })
    }
}

pub struct PhaseH5 {
    file: File,
    date: String,
    raw_data: AnalogStream,
    sampling_frequency: f32,
    digital: Option<AnalogStream>,
    events: Vec<EventStream>,
    datalen: usize,
}

impl PhaseH5 {
    pub fn open(
        filename: &str,
    ) -> Result<Self, PhaseH5Error> {
        let file = File::open(filename, FileOpenAccess::ReadWrite)?;
        let data_group = file.open_group("Data")?;
        let analog_group =
            data_group.open_group("Recording_0/AnalogStream")?;

        let date = String::from_attribute(&data_group.open_attr("Date")?)?;

        let mut raw_data = None;
        let mut digital = None;
        let mut sampling_frequency = None;
        let mut datalen = None;

        for analog in analog_group.list_groups() {
            let (analog, is_raw) =
                AnalogStream::new(analog_group.open_group(&analog)?)?;
            if is_raw {
                if raw_data.is_none() {
                    if let Some(sf) = sampling_frequency {
                        if analog.sampling_frequency != sf {
                            return Err(PhaseH5Error::FileError(
                                    FileErrorType::AnalogChannelsHaveDifferentSamplingFrequencies));
                        }
                    } else {
                        sampling_frequency = Some(analog.sampling_frequency);
                    }

                    if let Some(dl) = datalen {
                        if analog.datalen != dl {
                            return Err(PhaseH5Error::FileError(
                                    FileErrorType::DigitalSamplesDoesNotMatchRawDataSamples));
                        }
                    } else {
                        datalen = Some(analog.datalen);
                    }

                    raw_data = Some(analog);
                } else {
                    return Err(PhaseH5Error::FileError(
                        FileErrorType::PhaseHasMultipleRawData,
                    ));
                }
            } else {
                if digital.is_none() {
                    if let Some(sf) = sampling_frequency {
                        if analog.sampling_frequency != sf {
                            return Err(PhaseH5Error::FileError(
                                    FileErrorType::AnalogChannelsHaveDifferentSamplingFrequencies));
                        }
                    } else {
                        sampling_frequency = Some(analog.sampling_frequency);
                    }

                    if let Some(dl) = datalen {
                        if analog.datalen != dl {
                            return Err(PhaseH5Error::FileError(
                                    FileErrorType::DigitalSamplesDoesNotMatchRawDataSamples));
                        }
                    } else {
                        datalen = Some(analog.datalen);
                    }

                    digital = Some(analog);
                } else {
                    return Err(PhaseH5Error::FileError(
                        FileErrorType::PhaseHasMultipleDigital,
                    ));
                }
            }
        }

        let mut events = vec![];

        if let Ok(events_group) =
            data_group.open_group("Recording_0/EventStream")
        {
            for event in events_group.list_groups() {
                events
                    .push(EventStream::new(events_group.open_group(&event)?)?);
            }
        }

        if raw_data.is_none() {
            return Err(PhaseH5Error::FileError(
                FileErrorType::PhaseHasNoRawData,
            ));
        }

        Ok(Self {
            file,
            date,
            raw_data: raw_data.unwrap(),
            sampling_frequency: sampling_frequency.unwrap(),
            datalen: datalen.unwrap(),
            digital,
            events,
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
        let mut labels = vec![];
        for ic in &self.raw_data.info_channels {
            labels.push(cchar_to_string!(ic.label));
        }
        labels
    }

    fn raw_data(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<Vec<f32>, SpikeError> {
        todo!()
    }

    fn set_raw_data(
        &mut self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
        data: &[f32],
    ) -> Result<(), SpikeError> {
        todo!()
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
        todo!()
    }

    fn set_digital(
        &mut self,
        index: usize,
        start: Option<usize>,
        end: Option<usize>,
        data: &[f32],
    ) -> Result<(), SpikeError> {
        todo!()
    }

    fn n_events(&self) -> usize {
        todo!()
    }

    fn events(&self, index: usize) -> Result<Vec<u64>, SpikeError> {
        todo!()
    }

    fn peak_train(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<(Vec<f32>, Vec<usize>), SpikeError> {
        todo!()
    }

    fn set_peak_train(
        &mut self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
        data: (Vec<f32>, Vec<usize>),
    ) -> Result<(), SpikeError> {
        todo!()
    }
}

impl std::fmt::Display for PhaseH5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PhaseH5:")?;
        writeln!(f, "  Recording date: {}", self.date)?;
        writeln!(
            f,
            "  Raw data channels: {}",
            self.raw_data.info_channels.len()
        )?;
        writeln!(f, "  Datalen: {}", self.datalen)?;
        writeln!(f, "  Sampling Frequency: {}", self.sampling_frequency)?;
        writeln!(f, "  Digital present: {}", self.digital.is_some())?;
        Ok(())
    }
}
