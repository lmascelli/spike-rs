use hdf5_rs::types::{AttrOpener, AttributeFillable, File, FileOpenAccess, Group, GroupOpener};

use hdf5_rs::error::Error as H5Error;
use spike_rs::{error::SpikeError, types::PhaseHandler};

mod info_channel;

pub enum FileErrorType {
    AnalogUnknownType,
    PhaseHasNoRawData,
    PhaseHasMultipleRawData,
    PhaseHasMultipleDigital,
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
    group: Group,
}

impl AnalogStream {
    fn new(group: Group) -> Result<(Self, bool), PhaseH5Error> {
        let label = String::from_attribute(&group.open_attr("Label")?)?;
        if label.contains("Electrode Raw Data") {
            Ok((Self { group }, true))
        } else if label.contains("Digital Data") {
            Ok((Self { group }, false))
        } else {
            Err(PhaseH5Error::FileError(FileErrorType::AnalogUnknownType))
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
    digital: Option<AnalogStream>,
    events: Vec<EventStream>,
}

impl PhaseH5 {
    pub fn open(filename: &str) -> Result<Self, PhaseH5Error> {
        let file = File::open(filename, FileOpenAccess::ReadWrite)?;
        let data_group = file.open_group("Data")?;
        let analog_group = data_group.open_group("Recording_0/AnalogStream")?;

        let date = String::from_attribute(&data_group.open_attr("Date")?)?;

        let mut raw_data = None;
        let mut digital = None;

        for analog in analog_group.list_groups() {
            let (analog, is_raw) = AnalogStream::new(analog_group.open_group(&analog)?)?;
            if is_raw {
                if raw_data.is_none() {
                    raw_data = Some(analog);
                } else {
                    return Err(PhaseH5Error::FileError(
                        FileErrorType::PhaseHasMultipleRawData,
                    ));
                }
            } else {
                if digital.is_none() {
                    digital = Some(analog);
                } else {
                    return Err(PhaseH5Error::FileError(
                        FileErrorType::PhaseHasMultipleDigital,
                    ));
                }
            }
        }

        let mut events = vec![];

        if let Ok(events_group) = data_group.open_group("Recording_0/EventStream") {
            for event in events_group.list_groups() {
                events.push(EventStream::new(events_group.open_group(&event)?)?);
            }
        }

        if raw_data.is_none() {
            return Err(PhaseH5Error::FileError(FileErrorType::PhaseHasNoRawData));
        }

        Ok(Self {
            file,
            date,
            raw_data: raw_data.unwrap(),
            digital,
            events,
        })
    }
}

impl PhaseHandler for PhaseH5 {
    fn sampling_frequency(&self) -> f32 {
        todo!()
    }

    fn datalen(&self) -> usize {
        todo!()
    }

    fn labels(&self) -> Vec<String> {
        todo!()
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
        todo!()
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
