use std::collections::HashMap;
use std::ffi::{CStr, CString};

mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn spike_c_init() {
    unsafe { sys::pycodeh5_init() };
}

pub fn spike_c_close() {
    unsafe { sys::pycodeh5_close() };
}

#[derive(Debug)]
pub enum Error {
    ErrorNotYetConverted,
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
    PeakTrainLenOpenValuesDataspace,
    PeakTrainLenGetValuesDataspace,
    PeakTrainLenOpenSamplesDataspace,
    PeakTrainLenGetSamplesDataspace,
    PeakTrainLenValuesSamplesDifferent,
    PeakTrainLenCloseValuesDataset,
    PeakTrainLenCloseSamplesDataset,
    PeakTrainCreateMemoryDataspace,
    PeakTrainReadValuesDataset,
    PeakTrainReadSamplesDataset,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{:?}", self)?;
        Ok(())
    }
}
impl std::error::Error for Error {}

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
            sys::phaseh5_error_EVENTS_CREATE_MEMORY_DATASPACE_FAIL => Err(Error::EventsCreateMemoryDataspace),
            sys::phaseh5_error_EVENTS_READ_DATASET_FAIL => Err(Error::EventsReadDataset),
            sys::phaseh5_error_PEAK_TRAIN_NO_PEAK_GROUP => Err(Error::PeakTrainNoPeakGroup),
            sys::phaseh5_error_PEAK_TRAIN_VALUES_DATASET_LINK_FAIL => Err(Error::PeakTrainValuesDatasetLink),
            sys::phaseh5_error_PEAK_TRAIN_NO_VALUES_DATASET => Err(Error::PeakTrainNoValuesDataset),
            sys::phaseh5_error_PEAK_TRAIN_SAMPLES_DATASET_LINK_FAIL => Err(Error::PeakTrainSamplesDatasetLink),
            sys::phaseh5_error_PEAK_TRAIN_NO_SAMPLES_DATASET => Err(Error::PeakTrainNoSamplesDataset),
            sys::phaseh5_error_PEAK_TRAIN_OPEN_VALUES_DATASET_FAIL => Err(Error::PeakTrainOpenValuesDataset),
            sys::phaseh5_error_PEAK_TRAIN_OPEN_SAMPLES_DATASET_FAIL => Err(Error::PeakTrainOpenSamplesDataset),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_VALUES_DATASET_LINK_FAIL => Err(Error::DeletePeakTrainValuesDatasetLink),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_NO_VALUES_DATASET => Err(Error::DeletePeakTrainNoValuesDataset),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_SAMPLES_DATASET_LINK_FAIL => Err(Error::DeletePeakTrainSamplesDatasetLink),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_NO_SAMPLES_DATASET => Err(Error::DeletePeakTrainNoSamplesDataset),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_VALUES_DATASET_FAIL => Err(Error::DeletePeakTrainValuesDataset),
            sys::phaseh5_error_DELETE_PEAK_TRAIN_SAMPLES_DATASET_FAIL => Err(Error::DeletePeakTrainSamplesDataset),
            sys::phaseh5_error_PEAK_TRAIN_LEN_OPEN_VALUES_DATASPACE_FAIL => Err(Error::PeakTrainLenOpenValuesDataspace),
            sys::phaseh5_error_PEAK_TRAIN_LEN_OPEN_SAMPLES_DATASPACE_FAIL => Err(Error::PeakTrainLenOpenSamplesDataspace),
            sys::phaseh5_error_PEAK_TRAIN_LEN_GET_VALUES_DATASPACE_DIM_FAIL => Err(Error::PeakTrainLenGetValuesDataspace),
            sys::phaseh5_error_PEAK_TRAIN_LEN_OPEN_SAMPLES_DATASPACE_FAIL => Err(Error::PeakTrainLenOpenSamplesDataspace),
            sys::phaseh5_error_PEAK_TRAIN_LEN_GET_SAMPLES_DATASPACE_DIM_FAIL => Err(Error::PeakTrainLenGetSamplesDataspace),
            sys::phaseh5_error_PEAK_TRAIN_LEN_VALUES_SAMPLES_DIFFERENT => Err(Error::PeakTrainLenValuesSamplesDifferent),
            sys::phaseh5_error_PEAK_TRAIN_LEN_CLOSE_VALUES_DATASET_FAIL => Err(Error::PeakTrainLenCloseValuesDataset),
            sys::phaseh5_error_PEAK_TRAIN_LEN_CLOSE_SAMPLES_DATASET_FAIL => Err(Error::PeakTrainLenCloseSamplesDataset),
            sys::phaseh5_error_PEAK_TRAIN_CREATE_MEMORY_DATASPACE_FAIL => Err(Error::PeakTrainCreateMemoryDataspace),
            sys::phaseh5_error_PEAK_TRAIN_READ_VALUES_DATASET_FAIL => Err(Error::PeakTrainReadValuesDataset),
            sys::phaseh5_error_PEAK_TRAIN_READ_SAMPLES_DATASET_FAIL => Err(Error::PeakTrainReadSamplesDataset),
            _ => Err(Error::ErrorNotYetConverted),
        }
    }
}

pub struct PeakTrain {
    samples: Vec<i64>,
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
            samples: self.samples.as_mut_ptr(),
            values: self.values.as_mut_ptr(),
        }
    }
}

macro_rules! peak_train_ptr {
    ($p:ident) => (&mut$p as *mut sys::PeakTrain)
}

pub struct Phase {
    pub filename: String,
    pub labels_map: HashMap<String, usize>,
    phase: sys::PhaseH5,
}

macro_rules! phase_ptr  {
    ($p:ident) => (&$p.phase as *const sys::PhaseH5 as *mut sys::PhaseH5)
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
            Err(err) => Err(err),
        }
    }

    pub fn datalen(&self) -> usize {
        return self.phase.datalen;
    }

    pub fn sampling_frequency(&self) -> f32 {
        return self.phase.sampling_frequency;
    }

    pub fn labels(&self) -> Vec<String> {
        let mut ret = vec![];

        for (label, _index) in &self.labels_map {
            ret.push(label.clone());
        }

        ret
    }

    pub fn n_digitals(&self) -> usize {
        if self.phase.has_digital {
            1
        } else {
            0
        }
    }

    pub fn n_events(&self) -> usize {
        self.phase.n_events as usize
    }

    pub fn events_len(&self, index: usize) -> usize {
        let mut dims = 0u64;
        unsafe {
            sys::events_len(phase_ptr!(self), index, &mut dims as *mut _);
        }
        dims as usize
    }

    pub fn raw_data(&self, label: &str, start: Option<usize>, end: Option<usize>) -> Vec<f32> {
        let actual_start = match start {
            Some(val) => val,
            None => 0,
        };
        let actual_end = match end {
            Some(val) => val,
            None => self.datalen() - 2,
        };

        if actual_start >= actual_end {
            panic!("raw_data: [start] is not before [end]");
        }

        if !self.labels_map.contains_key(label) {
            panic!("raw_data: Label not found");
        }
        let index = self.labels_map[label];

        let mut ret = vec![0; actual_end - actual_start];
        
        let res = Error::from_phaseh5_error(unsafe {sys::raw_data(phase_ptr!(self), index, actual_start, actual_end, ret.as_mut_ptr().cast())});

        match res {
            Ok(()) => {
                let conversion_factor = self.phase.raw_data.info_channels[index].conversion_factor as f32 *
                    f32::powf(10f32, self.phase.raw_data.info_channels[index].exponent as f32);
                let offset = self.phase.raw_data.info_channels[index].ad_zero;

                ret.iter().map(|x| (*x - offset) as f32 * conversion_factor).collect()
            },
            Err(err) => {
                println!("{:?}", err);
                panic!("raw_data");
            }
        }
    }

    pub fn set_raw_data(&mut self, label: &str, data: Vec<f32>, start: Option<usize>) {
        let actual_start = match start {
            Some(val) => val,
            None => 0,
        };
        
        let actual_end = actual_start + data.len();
        if actual_end >= self.datalen() {
            panic!("set_raw_data: OutOfBounds");
        }

        if !self.labels_map.contains_key(label) {
            panic!("set_raw_data: Label not found");
        }
        let index = self.labels_map[label];

        let conversion_factor = self.phase.raw_data.info_channels[index].conversion_factor as f32 *
            f32::powf(10f32, self.phase.raw_data.info_channels[index].exponent as f32);
        let offset = self.phase.raw_data.info_channels[index].ad_zero;
        let mut buf : Vec<i32> = data.iter().map(|x| (*x / conversion_factor) as i32 + offset).collect();

        let res = unsafe {
            sys::set_raw_data(phase_ptr!(self), index, actual_start, actual_end, buf.as_mut_ptr())
        };

        match Error::from_phaseh5_error(res) {
            Ok(()) => (),
            Err(err) => {println!("{err:?}");}
        }
    }

    pub fn digital(&self, index: usize, start: Option<usize>, end: Option<usize>) -> Vec<f32> {
        if index > 0 {
            panic!("digital: no more than one index can be processed atm");
        }

        if self.phase.has_digital == false {
            panic!("digital: no digital present");
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
            panic!("raw_data: [start] is not before [end]");
        }

        let mut buf = vec![0f32; actual_end-actual_start];

        let res = unsafe { sys::digital(phase_ptr!(self), actual_start, actual_end, buf.as_mut_ptr().cast()) };
        match Error::from_phaseh5_error(res) {
            Ok(()) => buf,
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    pub fn set_digital(&self, index: usize, start: Option<usize>, mut data: Vec<f32>) -> Result<(), Error> {
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

        let res = unsafe { sys::set_digital(phase_ptr!(self), actual_start, actual_end, data.as_mut_ptr().cast()) };

        match Error::from_phaseh5_error(res) {
            Ok(()) => Ok(()),
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    pub fn events(&self, index: usize) -> Vec<i64> {
        let len = self.events_len(index);
        let mut data = vec![0i64; len];

        unsafe {
            sys::events(phase_ptr!(self), index, data.as_mut_ptr());
        }

        data
    }

    pub fn peak_train_len(&self, label: &str) -> usize {
        let label_c = CString::new(label).expect("peak_train_len: Failed to convert the CStr");
        let mut len = 0usize;
        let res = unsafe { sys::peak_train_len(phase_ptr!(self), label_c.as_ptr(), &mut len as *mut _) };

        match Error::from_phaseh5_error(res) {
            Ok(()) => len.try_into().unwrap(),
            Err(err) => { panic!("peak_train_len: {err:?}"); },
        } 
    }

    pub fn peak_train(&self, label: &str) -> (Vec<i64>, Vec<f32>) {
        let label_c = CString::new(label).expect("peak_train_len: Failed to convert the CStr");
        let peak_train_len = self.peak_train_len(label);
        let mut peak_train = PeakTrain::new(peak_train_len);
        let mut peak_train_c = peak_train.as_c_repr();
        let res = unsafe {
            sys::peak_train(phase_ptr!(self), label_c.as_ptr(), peak_train_ptr!(peak_train_c))
        };
        match Error::from_phaseh5_error(res) {
            Ok(()) => (peak_train.samples, peak_train.values),
            Err(err) => {
                panic!("{err}");
            }
        }
    }

    pub fn set_peak_train(&mut self, label: &str, peak_train: &PeakTrain) {
        
    }
}

impl Drop for Phase {
    fn drop(&mut self) {
        let _res = unsafe {
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
