// TODO! complete the set_peak_train
// TODO! add some functions error handling
// TODO! python wrapper

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use spike_rs::{
    error::SpikeError,
    types::PhaseHandler,
};

mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Debug)]
pub enum Error {
    ErrorNotYetConverted(i32),
    OpenFile,
    CloseFile,
    OpenDataGroup,
    OpenDateAttribute,
    ReadDateAttribute,
    OpenDateDatatype,
    OpenAnalogGroup,
    OpenInfoChannelDataset,
    OpenInfoChannelDataspace,
    OpenInfoChannelDatatype,
    ReadInfoChannels,
    OpenAnalogDataset,
    OpenLabelAttribute,
    ReadLabelAttribute,
    OpenLabelDatatype,
    ParseAnalogStream,
    MultipleDigitalStreams,
    MultipleRawDataStreams,
    MultipleSamplingFrequencies,
    MultipleDatalens,
    OpenChannelData,
    OpenChannelDataDataspace,
    GetChannelDataDims,
    NoRawDataStream,
    OpenEventStreamGroupLink,
    OpenEventStreamGroup,
    OpenEventStreamStream0GroupLink,
    MaxEventStreamsExceeded,
    OpenEntityDataset,
    EventEntityDatasetClose,
    OpenPeakTrainGroup,
    CreatePeakGroup,
    RawDataEndBeforeStart,
    RawDataEndOutOfBounds,
    RawDataGetDataspace,
    RawDataSelectHyperslab,
    RawDataCreateMemoryDataspace,
    RawDataReadData,
    SetRawDataEndOutOfBounds,
    SetRawDataGetDataspace,
    SetRawDataSelectHyperslab,
    SetRawDataCreateMemoryDataspace,
    SetRawDataWriteDataset,
    DigitalNoDigital,
    DigitalEndBeforeStart,
    DigitalEndOutOfBounds,
    DigitalGetDataspaceFail,
    DigitalSelectHyperslabFail,
    DigitalCreateMemoryDataspaceFail,
    DigitalReadDataFail,
    SetDigitalNoDigital,
    SetDigitalEndBeforeStart,
    SetDigitalEndOutOfBounds,
    SetDigitalGetDataspaceFail,
    SetDigitalSelectHyperslabFail,
    SetDigitalCreateMemoryDataspaceFail,
    SetDigitalWriteDataFail,
    EventsLenIndexOutOfBounds,
    EventsLenOpenEventDataspace,
    EventsLenGetDims,
    EventsIndexOutOfBounds,
    EventsGetEventsDataspace,
    EventsSelectDataspaceHyperslab,
    EventsCreateMemoryDataspace,
    EventsReadDataset,
    PeakTrainNoPeakGroup,
    PeakTrainValuesDatasetLink,
    PeakTrainNoValuesDataset,
    PeakTrainSamplesDatasetLink,
    PeakTrainNoSamplesDataset,
    PeakTrainOpenValuesDataset,
    PeakTrainOpenSamplesDataset,
    DeletePeakTrainValuesDatasetLink,
    DeletePeakTrainNoValuesDataset,
    DeletePeakTrainSamplesDatasetLink,
    DeletePeakTrainNoSamplesDataset,
    DeletePeakTrainValuesDataset,
    DeletePeakTrainSamplesDataset,
    PeakTrainCloseMemoryDataspaceFail,
    PeakTrainLenOpenValuesDataset,
    PeakTrainLenOpenSamplesDataset,
    PeakTrainLenOpenSamplesDataspace,
    PeakTrainLenOpenValuesDataspace,
    PeakTrainLenGetValuesDataspace,
    PeakTrainLenGetSamplesDataspace,
    PeakTrainLenValuesSamplesDifferent,
    PeakTrainLenCloseSamplesDataset,
    PeakTrainLenCloseValuesDataset,
    PeakTrainLenCloseSamplesDataspace,
    PeakTrainLenCloseValuesDataspace,
    PeakTrainCloseValuesDataset,
    PeakTrainCloseSamplesDataset,
    PeakTrainCreateMemoryDataspace,
    PeakTrainReadValuesDataset,
    PeakTrainReadSamplesDataset,
    SetPeakTrainCheckLabelGroup,
    SetPeakTrainCloseDeletedValuesDataset,
    SetPeakTrainCloseDeletedSamplesDataset,
    SetPeakTrainCloseSamplesFileDataspace,
    SetPeakTrainCloseValuesFileDataspace,
    SetPeakTrainCreateSamplesMemoryDataspace,
    SetPeakTrainCreateValuesMemoryDataspace,
    SetPeakTrainCreateSamplesFileDataspace,
    SetPeakTrainCreateValuesFileDataspace,
    SetPeakTrainCreateSamplesMemoryDataset,
    SetPeakTrainCreateValuesMemoryDataset,
    SetPeakTrainWriteSamplesDataset,
    SetPeakTrainWriteValuesDataset,
    SetPeakTrainCloseSamplesMemoryDataspace,
    SetPeakTrainCloseValuesMemoryDataspace,
    SetPeakTrainCloseSamplesDataset,
    SetPeakTrainCloseValuesDataset,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{:?}", self)?;
        Ok(())
    }
}
impl std::error::Error for Error {}

impl From<Error> for SpikeError {
    fn from(err: Error) -> Self {
        SpikeError::Implementation(format!("{:?}", err))
    }
}

impl Error {
    fn from_phaseh5_error(code: sys::phaseh5_error) -> Result<(), Self> {
        match code {
            sys::phaseh5_error_OK => Ok(()),
            sys::phaseh5_error_OPEN_FAIL => Err(Error::OpenFile),
            sys::phaseh5_error_CLOSE_FILE_FAIL => Err(Error::CloseFile),
            sys::phaseh5_error_OPEN_DATA_GROUP_FAIL => Err(Error::OpenDataGroup),
            sys::phaseh5_error_OPEN_DATE_ATTRIBUTE_FAIL => Err(Error::OpenDateAttribute),
            sys::phaseh5_error_READ_DATE_ATTRIBUTE_FAIL => Err(Error::ReadDateAttribute),
            sys::phaseh5_error_OPEN_DATE_DATATYPE_FAIL => Err(Error::OpenDateDatatype),
            sys::phaseh5_error_OPEN_ANALOG_GROUP_FAIL => Err(Error::OpenAnalogGroup),
            sys::phaseh5_error_OPEN_INFO_CHANNEL_DATASET_FAIL => Err(Error::OpenInfoChannelDataset),
            sys::phaseh5_error_OPEN_INFO_CHANNEL_DATASPACE_FAIL => Err(Error::OpenInfoChannelDataspace),
            sys::phaseh5_error_OPEN_INFO_CHANNEL_DATATYPE_FAIL => Err(Error::OpenInfoChannelDatatype),
            sys::phaseh5_error_OPEN_ANALOG_DATASET_FAIL => Err(Error::OpenAnalogDataset),
            sys::phaseh5_error_OPEN_LABEL_ATTRIBUTE_FAIL => Err(Error::OpenLabelAttribute),
            sys::phaseh5_error_READ_LABEL_ATTRIBUTE_FAIL => Err(Error::ReadLabelAttribute),
            sys::phaseh5_error_OPEN_LABEL_DATATYPE_FAIL => Err(Error::OpenLabelDatatype),
            sys::phaseh5_error_READ_INFO_CHANNELS_FAIL => Err(Error::ReadInfoChannels),
            sys::phaseh5_error_PARSE_ANALOG_STREAM_DIFFERENT_TICK => Err(Error::ParseAnalogStream),
            sys::phaseh5_error_MULTIPLE_DIGITAL_STREAMS => Err(Error::MultipleDigitalStreams),
            sys::phaseh5_error_MULTIPLE_RAW_DATA_STREAMS => Err(Error::MultipleRawDataStreams),
            sys::phaseh5_error_MULTIPLE_SAMPLING_FREQUENCIES => Err(Error::MultipleSamplingFrequencies),
            sys::phaseh5_error_MULTIPLE_DATALENS => Err(Error::MultipleDatalens),
            sys::phaseh5_error_OPEN_CHANNEL_DATA_FAIL => Err(Error::OpenChannelData),
            sys::phaseh5_error_OPEN_CHANNEL_DATA_DATASPACE_FAIL => Err(Error::OpenChannelDataDataspace),
            sys::phaseh5_error_GET_CHANNEL_DATA_DIMS_FAIL => Err(Error::GetChannelDataDims),
            sys::phaseh5_error_NO_RAW_DATA_STREAM => Err(Error::NoRawDataStream),
            sys::phaseh5_error_OPEN_EVENT_STREAM_GROUP_FAIL => Err(Error::OpenEventStreamGroup),
            sys::phaseh5_error_OPEN_EVENT_STREAM_GROUP_LINK_FAIL => Err(Error::OpenEventStreamGroupLink),
            sys::phaseh5_error_OPEN_EVENT_STREAM_STREAM_0_GROUP_LINK_FAIL => Err(Error::OpenEventStreamStream0GroupLink),
            sys::phaseh5_error_MAX_EVENT_STREAMS_EXCEEDED => Err(Error::MaxEventStreamsExceeded),
            sys::phaseh5_error_OPEN_ENTITY_DATASET_FAIL => Err(Error::OpenEntityDataset),
            sys::phaseh5_error_EVENT_ENTITY_DATASET_CLOSE_FAIL => Err(Error::EventEntityDatasetClose),
            sys::phaseh5_error_OPEN_PEAK_TRAIN_GROUP_FAIL => Err(Error::OpenPeakTrainGroup),
            sys::phaseh5_error_CREATE_PEAK_GROUP_FAIL => Err(Error::CreatePeakGroup),
            sys::phaseh5_error_RAW_DATA_END_BEFORE_START => Err(Error::RawDataEndBeforeStart),
            sys::phaseh5_error_RAW_DATA_END_OUT_OF_BOUNDS => Err(Error::RawDataEndOutOfBounds),
            sys::phaseh5_error_RAW_DATA_GET_DATASPACE_FAIL => Err(Error::RawDataGetDataspace),
            sys::phaseh5_error_RAW_DATA_SELECT_HYPERSLAB_FAIL => Err(Error::RawDataSelectHyperslab),
            sys::phaseh5_error_RAW_DATA_CREATE_MEMORY_DATASPACE_FAIL => Err(Error::RawDataCreateMemoryDataspace),
            sys::phaseh5_error_RAW_DATA_READ_DATA_FAIL => Err(Error::RawDataReadData),
            sys::phaseh5_error_SET_RAW_DATA_GET_DATASPACE_FAIL => Err(Error::SetRawDataGetDataspace),
            sys::phaseh5_error_SET_RAW_DATA_SELECT_HYPERSLAB_FAIL => Err(Error::SetRawDataSelectHyperslab),
            sys::phaseh5_error_SET_RAW_DATA_CREATE_MEMORY_DATASPACE_FAIL => Err(Error::SetRawDataCreateMemoryDataspace),
            sys::phaseh5_error_SET_RAW_DATA_WRITE_DATASET_FAIL => Err(Error::SetRawDataWriteDataset),
            sys::phaseh5_error_DIGITAL_NO_DIGITAL => Err(Error::DigitalNoDigital),
            sys::phaseh5_error_DIGITAL_END_BEFORE_START => Err(Error::DigitalEndBeforeStart),
            sys::phaseh5_error_DIGITAL_END_OUT_OF_BOUNDS => Err(Error::DigitalEndOutOfBounds),
            sys::phaseh5_error_DIGITAL_GET_DATASPACE_FAIL => Err(Error::DigitalGetDataspaceFail),
            sys::phaseh5_error_DIGITAL_SELECT_HYPERSLAB_FAIL => Err(Error::DigitalSelectHyperslabFail),
            sys::phaseh5_error_DIGITAL_CREATE_MEMORY_DATASPACE_FAIL => Err(Error::DigitalCreateMemoryDataspaceFail),
            sys::phaseh5_error_DIGITAL_READ_DATA_FAIL => Err(Error::DigitalReadDataFail),
            sys::phaseh5_error_SET_DIGITAL_NO_DIGITAL => Err(Error::SetDigitalNoDigital),
            sys::phaseh5_error_SET_DIGITAL_END_BEFORE_START => Err(Error::SetDigitalEndBeforeStart),
            sys::phaseh5_error_SET_DIGITAL_END_OUT_OF_BOUNDS => Err(Error::SetDigitalEndOutOfBounds),
            sys::phaseh5_error_SET_DIGITAL_GET_DATASPACE_FAIL => Err(Error::SetDigitalGetDataspaceFail),
            sys::phaseh5_error_SET_DIGITAL_SELECT_HYPERSLAB_FAIL => Err(Error::SetDigitalSelectHyperslabFail),
            sys::phaseh5_error_SET_DIGITAL_CREATE_MEMORY_DATASPACE_FAIL => Err(Error::SetDigitalCreateMemoryDataspaceFail),
            sys::phaseh5_error_SET_DIGITAL_WRITE_DATA_FAIL => Err(Error::SetDigitalWriteDataFail),
            sys::phaseh5_error_EVENTS_LEN_INDEX_OUT_OF_BOUNDS => Err(Error::EventsLenIndexOutOfBounds),
            sys::phaseh5_error_EVENTS_LEN_OPEN_EVENT_DATASPACE_FAIL => Err(Error::EventsLenOpenEventDataspace),
            sys::phaseh5_error_EVENTS_INDEX_OUT_OF_BOUNDS => Err(Error::EventsIndexOutOfBounds),
            sys::phaseh5_error_EVENTS_LEN_GET_DIMS_FAIL => Err(Error::EventsLenGetDims),
            sys::phaseh5_error_EVENTS_GET_EVENTS_DATASPACE_FAIL => Err(Error::EventsGetEventsDataspace),
            sys::phaseh5_error_EVENTS_SELECT_DATASPACE_HYPERSLAB_FAIL => Err(Error::EventsSelectDataspaceHyperslab),
            sys::phaseh5_error_PEAK_TRAIN_CLOSE_MEMORY_DATASPACE_FAIL => Err(Error::PeakTrainCloseMemoryDataspaceFail),
            sys::phaseh5_error_PEAK_TRAIN_LEN_CLOSE_VALUES_DATASPACE_FAIL => Err(Error::PeakTrainLenCloseValuesDataspace),
            sys::phaseh5_error_PEAK_TRAIN_LEN_CLOSE_VALUES_DATASET_FAIL => Err(Error::PeakTrainLenCloseValuesDataset),
            sys::phaseh5_error_PEAK_TRAIN_LEN_CLOSE_SAMPLES_DATASPACE_FAIL => Err(Error::PeakTrainLenCloseSamplesDataspace),
            sys::phaseh5_error_PEAK_TRAIN_LEN_CLOSE_SAMPLES_DATASET_FAIL => Err(Error::PeakTrainLenCloseSamplesDataset),
            sys::phaseh5_error_EVENTS_CREATE_MEMORY_DATASPACE_FAIL => Err(Error::EventsCreateMemoryDataspace),
            sys::phaseh5_error_EVENTS_READ_DATASET_FAIL => Err(Error::EventsReadDataset),
            sys::phaseh5_error_PEAK_TRAIN_NO_PEAK_GROUP => Err(Error::PeakTrainNoPeakGroup),
            sys::phaseh5_error_PEAK_TRAIN_VALUES_DATASET_LINK_FAIL => Err(Error::PeakTrainValuesDatasetLink),
            sys::phaseh5_error_PEAK_TRAIN_NO_VALUES_DATASET => Err(Error::PeakTrainNoValuesDataset),
            sys::phaseh5_error_PEAK_TRAIN_SAMPLES_DATASET_LINK_FAIL => Err(Error::PeakTrainSamplesDatasetLink),
            sys::phaseh5_error_PEAK_TRAIN_NO_SAMPLES_DATASET => Err(Error::PeakTrainNoSamplesDataset),
            sys::phaseh5_error_PEAK_TRAIN_OPEN_VALUES_DATASET_FAIL => Err(Error::PeakTrainOpenValuesDataset),
            sys::phaseh5_error_PEAK_TRAIN_OPEN_SAMPLES_DATASET_FAIL => Err(Error::PeakTrainOpenSamplesDataset),
            sys::phaseh5_error_PEAK_TRAIN_CLOSE_VALUES_DATASET_FAIL => Err(Error::PeakTrainCloseValuesDataset),
            sys::phaseh5_error_PEAK_TRAIN_CLOSE_SAMPLES_DATASET_FAIL => Err(Error::PeakTrainCloseSamplesDataset),
            sys::phaseh5_error_SET_PEAK_TRAIN_CHECK_LABEL_GROUP_FAIL => Err(Error::SetPeakTrainCheckLabelGroup),
            sys::phaseh5_error_SET_PEAK_TRAIN_CLOSE_DELETED_VALUES_DATASET_FAIL => Err(Error::SetPeakTrainCloseDeletedValuesDataset),
            sys::phaseh5_error_SET_PEAK_TRAIN_CLOSE_DELETED_SAMPLES_DATASET_FAIL => Err(Error::SetPeakTrainCloseDeletedSamplesDataset),
            sys::phaseh5_error_SET_PEAK_TRAIN_CLOSE_SAMPLES_FILE_DATASPACE_FAIL => Err(Error::SetPeakTrainCloseSamplesFileDataspace),
            sys::phaseh5_error_SET_PEAK_TRAIN_CLOSE_VALUES_FILE_DATASPACE_FAIL => Err(Error::SetPeakTrainCloseValuesFileDataspace),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_VALUES_DATASET_LINK_FAIL => Err(Error::DeletePeakTrainValuesDatasetLink),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_NO_VALUES_DATASET => Err(Error::DeletePeakTrainNoValuesDataset),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_SAMPLES_DATASET_LINK_FAIL => Err(Error::DeletePeakTrainSamplesDatasetLink),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_NO_SAMPLES_DATASET => Err(Error::DeletePeakTrainNoSamplesDataset),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_VALUES_DATASET_FAIL => Err(Error::DeletePeakTrainValuesDataset),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_SAMPLES_DATASET_FAIL => Err(Error::DeletePeakTrainSamplesDataset),
            sys::phaseh5_error_PEAK_TRAIN_LEN_GET_VALUES_DATASPACE_DIM_FAIL => Err(Error::PeakTrainLenGetValuesDataspace),
            sys::phaseh5_error_PEAK_TRAIN_LEN_CLOSE_VALUES_DATASPACE_FAIL => Err(Error::PeakTrainLenCloseValuesDataspace),
            sys::phaseh5_error_PEAK_TRAIN_LEN_OPEN_VALUES_DATASPACE_FAIL => Err(Error::PeakTrainLenOpenValuesDataspace),
            sys::phaseh5_error_PEAK_TRAIN_LEN_GET_SAMPLES_DATASPACE_DIM_FAIL => Err(Error::PeakTrainLenGetSamplesDataspace),
            sys::phaseh5_error_PEAK_TRAIN_LEN_CLOSE_SAMPLES_DATASET_FAIL => Err(Error::PeakTrainLenCloseValuesDataset),
            sys::phaseh5_error_PEAK_TRAIN_LEN_VALUES_SAMPLES_DIFFERENT => Err(Error::PeakTrainLenValuesSamplesDifferent),
            sys::phaseh5_error_PEAK_TRAIN_LEN_CLOSE_VALUES_DATASET_FAIL => Err(Error::PeakTrainLenCloseValuesDataset),
            sys::phaseh5_error_PEAK_TRAIN_CREATE_MEMORY_DATASPACE_FAIL => Err(Error::PeakTrainCreateMemoryDataspace),
            sys::phaseh5_error_PEAK_TRAIN_READ_VALUES_DATASET_FAIL => Err(Error::PeakTrainReadValuesDataset),
            sys::phaseh5_error_PEAK_TRAIN_READ_SAMPLES_DATASET_FAIL => Err(Error::PeakTrainReadSamplesDataset),
            sys::phaseh5_error_SET_PEAK_TRAIN_CREATE_SAMPLES_MEMORY_DATASPACE_FAIL => Err(Error::SetPeakTrainCreateSamplesMemoryDataspace),
            sys::phaseh5_error_SET_PEAK_TRAIN_CREATE_VALUES_MEMORY_DATASPACE_FAIL => Err(Error::SetPeakTrainCreateValuesMemoryDataspace),
            sys::phaseh5_error_SET_PEAK_TRAIN_CREATE_SAMPLES_FILE_DATASET_FAIL => Err(Error::SetPeakTrainCreateSamplesMemoryDataset),
            sys::phaseh5_error_SET_PEAK_TRAIN_CREATE_VALUES_FILE_DATASET_FAIL => Err(Error::SetPeakTrainCreateValuesMemoryDataset),
            sys::phaseh5_error_SET_PEAK_TRAIN_CREATE_SAMPLES_FILE_DATASPACE_FAIL => Err(Error::SetPeakTrainCreateSamplesFileDataspace),
            sys::phaseh5_error_SET_PEAK_TRAIN_CREATE_VALUES_FILE_DATASPACE_FAIL => Err(Error::SetPeakTrainCreateValuesFileDataspace),
            sys::phaseh5_error_SET_PEAK_TRAIN_WRITE_SAMPLES_DATASET_FAIL => Err(Error::SetPeakTrainWriteSamplesDataset),
            sys::phaseh5_error_SET_PEAK_TRAIN_WRITE_VALUES_DATASET_FAIL => Err(Error::SetPeakTrainWriteValuesDataset),
            sys::phaseh5_error_SET_PEAK_TRAIN_CLOSE_SAMPLES_MEMORY_DATASPACE_FAIL => Err(Error::SetPeakTrainCloseSamplesMemoryDataspace),
            sys::phaseh5_error_SET_PEAK_TRAIN_CLOSE_VALUES_MEMORY_DATASPACE_FAIL => Err(Error::SetPeakTrainCloseValuesMemoryDataspace),
            sys::phaseh5_error_SET_PEAK_TRAIN_CLOSE_SAMPLES_DATASET_FAIL => Err(Error::SetPeakTrainCloseSamplesDataset),
            sys::phaseh5_error_SET_PEAK_TRAIN_CLOSE_VALUES_DATASET_FAIL => Err(Error::SetPeakTrainCloseValuesDataset),
            _ => Err(Error::ErrorNotYetConverted(code.try_into().unwrap())),
        }
    }
}

pub fn spike_c_init() -> Result<(), Error> {
    Ok(Error::from_phaseh5_error(unsafe { sys::pycodeh5_init() })?)
}

pub fn spike_c_close() {
    unsafe { sys::pycodeh5_close() };
}

pub struct PeakTrain {
    samples: Vec<usize>,
    values: Vec<f32>,
}

impl PeakTrain {
    pub fn new(len: usize) -> Self {
        PeakTrain {
            samples: vec![0; len],
            values: vec![0f32; len],
        }
    }

    pub fn as_c_repr(&mut self) -> sys::PeakTrain {
        sys::PeakTrain {
            n_peaks: self.samples.len(),
            samples: self.samples.as_mut_ptr().cast(),
            values: self.values.as_mut_ptr(),
        }
    }
}

macro_rules! peak_train_ptr {
    ($p:ident) => (&mut$p as *mut sys::PeakTrain)
}

pub struct Phase {
    phase: sys::PhaseH5,
    pub filename: String,
    pub labels_map: HashMap<String, usize>,
}

macro_rules! phase_ptr  {
    ($p:ident) => (&$p.phase as *const sys::PhaseH5 as *mut sys::PhaseH5)
}

impl Drop for Phase {
    fn drop(&mut self) {
        unsafe {
            sys::phase_close(phase_ptr!(self));
        };
    }
}

impl std::fmt::Debug for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{{")?;
        writeln!(f, "  file: {},", self.filename)?;
        writeln!(f, "  datalen: {},", self.datalen())?;
        writeln!(f, "  sampling frequency: {},", self.sampling_frequency())?;
        writeln!(f, "  channels:")?;
        for label in self.labels() {
            writeln!(f, "    {}", label)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl std::default::Default for Phase {
    fn default() -> Self {
        Self {
            filename: String::new(),
            labels_map: HashMap::new(),
            phase: sys::PhaseH5 {
                fid: 0,
                date: [0; sys::DATE_STRING_LEN as usize],
                datalen: 0,
                sampling_frequency: 0f32,
                raw_data: sys::AnalogStream {
                    label: [0; sys::ANALOG_LABEL_STRING_LEN as usize],
                    n_channels: 0,
                    channel_data_dataset: 0,
                    datalen: 0,
                    info_channels: [sys::InfoChannel {
                        channel_id: 0,
                        row_index: 0,
                        group_id: 0,
                        electrode_group: 0,
                        label: std::ptr::null(),
                        raw_data_type: std::ptr::null(),
                        unit: std::ptr::null(),
                        exponent: 0,
                        ad_zero: 0,
                        tick: 0,
                        conversion_factor: 0,
                        adc_bits: 0,
                        high_pass_filter_type: std::ptr::null(),
                        high_pass_filter_cutoff: std::ptr::null(),
                        high_pass_filter_order: 0,
                        low_pass_filter_type: std::ptr::null(),
                        low_pass_filter_cutoff: std::ptr::null(),
                        low_pass_filter_order: 0,
                    };
                        sys::MAX_CHANNELS as usize],
                },
                has_digital: false,
                digital: sys::AnalogStream {
                    label: [0; sys::ANALOG_LABEL_STRING_LEN as usize],
                    n_channels: 0,
                    channel_data_dataset: 0,
                    datalen: 0,
                    info_channels: [sys::InfoChannel {
                        channel_id: 0,
                        row_index: 0,
                        group_id: 0,
                        electrode_group: 0,
                        label: std::ptr::null(),
                        raw_data_type: std::ptr::null(),
                        unit: std::ptr::null(),
                        exponent: 0,
                        ad_zero: 0,
                        tick: 0,
                        conversion_factor: 0,
                        adc_bits: 0,
                        high_pass_filter_type: std::ptr::null(),
                        high_pass_filter_cutoff: std::ptr::null(),
                        high_pass_filter_order: 0,
                        low_pass_filter_type: std::ptr::null(),
                        low_pass_filter_cutoff: std::ptr::null(),
                        low_pass_filter_order: 0,
                    };
                        sys::MAX_CHANNELS as usize],
                },
                n_events: 0,
                event_entities: [0; sys::MAX_EVENT_STREAMS as usize],
                peaks_group: 0,
            },
        }
    }
}

impl Phase {
    pub fn open(filename: &str) -> Result<Self, Error> {
        let mut phase = Self::default();
        phase.filename = filename.to_string();
        let cfilename = CString::new(filename).unwrap();

        let res = unsafe {
           sys::phase_open(
                phase_ptr!(phase),
                cfilename.as_ptr(),
            )
        };

        match Error::from_phaseh5_error(res){
            Ok(()) => {
                for i in 0..phase.phase.raw_data.n_channels as usize {
                    unsafe {
                        phase.labels_map.insert(CStr::from_ptr(phase.phase.raw_data.info_channels[i].label)
                            .to_str().expect("Failed to convert the CStr").to_string(), i);
                    }
                }
                Ok(phase)
            },
            Err(err) => {
                eprintln!("{err:?}");
                Err(err)   
            }
        }
    }

    pub fn events_len(&self, index: usize) -> usize {
        let mut dims = 0u64;
        unsafe {
            sys::events_len(phase_ptr!(self), index, &mut dims as *mut _);
        }
        dims as usize
    }

    pub fn peak_train_len(&self, label: &str) -> usize {
        let label_c = CString::new(label).expect("peak_train_len: Failed to convert the CStr");
        let mut len = 0usize;
        let res = unsafe { sys::peak_train_len(phase_ptr!(self), label_c.as_ptr(), &mut len as *mut _) };

        match Error::from_phaseh5_error(res) {
            Ok(()) => len,
            Err(err) => { panic!("peak_train_len: {err:?}"); },
        } 
    }
}

impl PhaseHandler for Phase {
    fn sampling_frequency(&self) -> f32 {
        return self.phase.sampling_frequency;
    }

    fn datalen(&self) -> usize {
        return self.phase.datalen;
    }

    fn labels(&self) -> Vec<String> {
        let mut ret = vec![];

        for (label, _index) in &self.labels_map {
            ret.push(label.clone());
        }

        ret
    }

    fn raw_data(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>
    ) -> Result<Vec<f32>, SpikeError> {
        let actual_start = match start {
            Some(val) => val,
            None => 0,
        };
        let actual_end = match end {
            Some(val) => val,
            None => self.datalen() - 2,
        };

        if actual_start >= actual_end {
            return Err(SpikeError::RawDataStartIsAfterEnd);
        }

        if actual_end >= self.datalen() {
            return Err(SpikeError::RawDataOutOfBounds);
        }

        if !self.labels_map.contains_key(channel) {
            return Err(SpikeError::RawDataLabelNotFound);
        }
        
        let index = self.labels_map[channel];

        let mut ret = vec![0; actual_end - actual_start];
        
        let res = Error::from_phaseh5_error(unsafe {sys::raw_data(phase_ptr!(self), index, actual_start, actual_end, ret.as_mut_ptr().cast())});

        match res {
            Ok(()) => {
                let conversion_factor = self.phase.raw_data.info_channels[index].conversion_factor as f32 *
                    f32::powf(10f32, self.phase.raw_data.info_channels[index].exponent as f32);
                let offset = self.phase.raw_data.info_channels[index].ad_zero;

                Ok(ret.iter().map(|x| (*x - offset) as f32 * conversion_factor).collect())
            },
            Err(err) => {
                Err(err.into())
            }
        }
    }

    fn set_raw_data(
        &mut self,
        channel: &str,
        start: Option<usize>,
        data: &[f32],
    ) -> Result<(), SpikeError> {
        let actual_start = match start {
            Some(val) => val,
            None => 0,
        };
        
        let actual_end = actual_start + data.len();

        if actual_end >= self.datalen() {
            return Err(SpikeError::SetRawDataOutOfBounds);
        }

        if !self.labels_map.contains_key(channel) {
            return Err(SpikeError::SetRawDataLabelNotFound);
        }

        let index = self.labels_map[channel];

        let conversion_factor = self.phase.raw_data.info_channels[index].conversion_factor as f32 *
            f32::powf(10f32, self.phase.raw_data.info_channels[index].exponent as f32);
        let offset = self.phase.raw_data.info_channels[index].ad_zero;
        let buf : Vec<i32> = data.iter().map(|x| (*x / conversion_factor) as i32 + offset).collect();

        let res = unsafe {
            sys::set_raw_data(phase_ptr!(self), index, actual_start, actual_end, buf.as_ptr())
        };

        Ok(Error::from_phaseh5_error(res)?)
    }

    fn n_digitals(&self) -> usize {
        if self.phase.has_digital {
            1
        } else {
            0
        }
    }

    fn digital(
        &self,
        index: usize,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<Vec<f32>, SpikeError> {
        if index > 0 {
            panic!("digital: no more than one index can be processed atm");
        }

        if self.phase.has_digital == false {
            return Err(SpikeError::DigitalNoDigitalPresent);
        }

        let actual_start = match start {
            Some(val) => val,
            None => 0,
        };

        let actual_end = match end {
            Some(val) => val,
            None => self.datalen() - 2,
        };

        if actual_start >= actual_end {
            return Err(SpikeError::DigitalStartIsAfterEnd);
        }

        let mut buf = vec![0f32; actual_end-actual_start];

        let res = unsafe { sys::digital(phase_ptr!(self), actual_start, actual_end, buf.as_mut_ptr().cast()) };

        match Error::from_phaseh5_error(res) {
            Ok(()) => Ok(buf),
            Err(err) => {
                Err(err.into())
            }
        }
    }

    fn set_digital(
        &mut self,
        index: usize,
        start: Option<usize>,
        data: &[f32],
    ) -> Result<(), SpikeError> {
        if index > 0 {
            panic!("set_digital: no more than one index can be processed atm");
        }

        if self.phase.has_digital == false {
            panic!("set_digital: no digital present");
        }

        let actual_start = match start {
            Some(val) => val,
            None => 0,
        };
        let actual_end = actual_start + data.len();

        if actual_end >= self.datalen() {
            panic!("set_digital: [end] is greater than [datalen]");
        }

        if actual_start >= actual_end {
            panic!("set_digital: [start] is not before [end]");
        }

        let res = unsafe { sys::set_digital(phase_ptr!(self), actual_start, actual_end, data.as_ptr().cast()) };

        match Error::from_phaseh5_error(res) {
            Ok(()) => Ok(()),
            Err(err) => {
                Err(err.into())
            }
        }
    }

    fn n_events(&self) -> usize {
        self.phase.n_events as usize
    }

    fn events(&self, index: usize) -> Result<Vec<i64>, SpikeError> { 
        let len = self.events_len(index);
        let mut data = vec![0i64; len];

        let res = unsafe {
            sys::events(phase_ptr!(self), index, data.as_mut_ptr())
        };
        
        match Error::from_phaseh5_error(res) {
            Ok(()) => Ok(data),
            Err(err) => Err(err.into())
        }
    }

    fn peak_train(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>
    ) -> Result<(Vec<usize>, Vec<f32>), SpikeError> {
        let channel_c = CString::new(channel).expect("peak_train_len: Failed to convert the CStr");
        let peak_train_len = self.peak_train_len(channel);
        let mut peak_train = PeakTrain::new(peak_train_len);
        let mut peak_train_c = peak_train.as_c_repr();
        let res = unsafe {
            sys::peak_train(phase_ptr!(self), channel_c.as_ptr(), peak_train_ptr!(peak_train_c))
        };

        match Error::from_phaseh5_error(res) {
            Ok(()) => {
                let (samples, values) = (peak_train.samples, peak_train.values);
                if start.is_none() && end.is_none() {
                    return Ok((samples, values));
                } else {
                    let start = start.unwrap_or(samples[0]);
                    let end = end.unwrap_or(samples[samples.len() - 1]);
                    let mut i_start = 0;
                    let mut i_end = samples.len() - 1;

                    for (i, val) in samples.iter().enumerate(){
                        if *val >= start {
                            i_start = i;
                            break;
                        }
                    }
                    for (i, val) in samples.iter().enumerate() {
                        if *val >= end {
                            i_end = i;
                            break;
                        }
                    }
                    Ok((samples[i_start..i_end].iter().map(|x| *x).collect(),
                        values[i_start..i_end].iter().map(|x| *x).collect()))
                }
            },
            Err(err) => {
                Err(err.into())
            }
        }
    }

    fn set_peak_train(
        &mut self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
        data: (Vec<usize>, Vec<f32>)
    ) -> Result<(), SpikeError> {
        let channel_c = CString::new(channel).expect("peak_train_len: Failed to convert the CStr");
        let peak_train_len = self.peak_train_len(channel);
        let mut peak_train = PeakTrain::new(peak_train_len);
        let mut peak_train_c = peak_train.as_c_repr();
        let res = unsafe {
            sys::peak_train(phase_ptr!(self), channel_c.as_ptr(), peak_train_ptr!(peak_train_c))
        };

        match Error::from_phaseh5_error(res) {
            Ok(()) => {
                // there is a group. Get the data and replace the new ones
                let (samples, values) = (peak_train.samples, peak_train.values);

                // the spikes train in present and contains data so the new data must be
                // inserted between start and stop positions
                let start = start.unwrap_or(samples[0]);
                let end = end.unwrap_or(samples[samples.len() - 1]);
                let mut i_start = 0;
                let mut i_end = samples.len() - 1;
                for (i, val) in samples.iter().enumerate() {
                    if *val >= start {
                        i_start = i;
                        break;
                    }
                }
                for (i, val) in samples.iter().enumerate() {
                    if *val >= end {
                        i_end = i;
                        break;
                    }
                }

                // get all values before start
                let before_start_samples = samples[0..i_start].to_vec();
                let before_start_values = values[0..i_start].to_vec();

                // get all values after end
                let after_end_samples = samples[i_end..].to_vec();
                let after_end_values = values[i_end..].to_vec();

                // join the values with data
                let mut new_samples = vec![];
                let mut new_values = vec![];

                new_samples
                    .extend_from_slice(before_start_samples.as_slice());
                new_samples.extend_from_slice(data.0.as_slice());
                new_samples.extend_from_slice(after_end_samples.as_slice());

                new_values
                    .extend_from_slice(before_start_values.as_slice());
                new_values.extend_from_slice(data.1.as_slice());
                new_values
                    .extend_from_slice(after_end_values.as_slice());

                // create the new PeakTrain
                let mut new_peak_train = PeakTrain::new(new_samples.len());
                let mut new_peak_train_c = new_peak_train.as_c_repr();

                // try to write it
                let res = unsafe {
                    sys::set_peak_train(phase_ptr!(self), channel_c.as_ptr(), peak_train_ptr!(new_peak_train_c))
                };

                match Error::from_phaseh5_error(res) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(err.into()),
                }
            },

            Err(Error::PeakTrainNoPeakGroup) => {
                // there is no group yet. Just pass the new data
                let res = unsafe {
                    sys::set_peak_train(phase_ptr!(self), channel_c.as_ptr(), peak_train_ptr!(peak_train_c))
                };
                match Error::from_phaseh5_error(res) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(err.into()),
                }
            },

            Err(err) => {
                Err(err.into())
            }
        }
    }
}
