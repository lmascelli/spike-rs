use std::ffi::CString;
mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Debug)]
pub enum Error {
    ErrorNotYetConverted,
    OpenFile,
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
    RawDataEndBeforeStart,
    RawDataEndOutOfBounds,
    RawDataGetDataspace,
    RawDataSelectHyperslab,
    RawDataCreateMemoryDataspace,
    RawDataReadData,
    SetRawDataGetDataspace,
    SetRawDataSelectHyperslab,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{:?}", self)?;
        Ok(())
    }
}

impl std::error::Error for Error {}

impl Error {
    fn from_phaseh5_error(code: sys::phaseh5_error) -> Self {
        match code {
            sys::phaseh5_error_OPEN_FAIL => Error::OpenFile,
            sys::phaseh5_error_OPEN_DATA_GROUP_FAIL => Error::OpenDataGroup,
            sys::phaseh5_error_OPEN_DATE_ATTRIBUTE_FAIL => Error::OpenDateAttribute,
            sys::phaseh5_error_READ_DATE_ATTRIBUTE_FAIL => Error::ReadDateAttribute,
            sys::phaseh5_error_OPEN_DATE_DATATYPE_FAIL => Error::OpenDateDatatype,
            sys::phaseh5_error_OPEN_ANALOG_GROUP_FAIL => Error::OpenAnalogGroup,
            sys::phaseh5_error_OPEN_INFO_CHANNEL_DATASET_FAIL => Error::OpenInfoChannelDataset,
            sys::phaseh5_error_OPEN_INFO_CHANNEL_DATASPACE_FAIL => Error::OpenInfoChannelDataspace,
            sys::phaseh5_error_OPEN_INFO_CHANNEL_DATATYPE_FAIL => Error::OpenInfoChannelDatatype,
            sys::phaseh5_error_OPEN_ANALOG_DATASET_FAIL => Error::OpenAnalogDataset,
            sys::phaseh5_error_OPEN_LABEL_ATTRIBUTE_FAIL => Error::OpenLabelAttribute,
            sys::phaseh5_error_READ_LABEL_ATTRIBUTE_FAIL => Error::ReadLabelAttribute,
            sys::phaseh5_error_OPEN_LABEL_DATATYPE_FAIL => Error::OpenLabelDatatype,
            sys::phaseh5_error_READ_INFO_CHANNELS_FAIL => Error::ReadInfoChannels,
            sys::phaseh5_error_PARSE_ANALOG_STREAM_DIFFERENT_TICK => Error::ParseAnalogStream,
            sys::phaseh5_error_MULTIPLE_DIGITAL_STREAMS => Error::MultipleDigitalStreams,
            sys::phaseh5_error_MULTIPLE_RAW_DATA_STREAMS => Error::MultipleRawDataStreams,
            sys::phaseh5_error_MULTIPLE_SAMPLING_FREQUENCIES => Error::MultipleSamplingFrequencies,
            sys::phaseh5_error_MULTIPLE_DATALENS => Error::MultipleDatalens,
            sys::phaseh5_error_OPEN_CHANNEL_DATA_FAIL => Error::OpenChannelData,
            sys::phaseh5_error_OPEN_CHANNEL_DATA_DATASPACE_FAIL => Error::OpenChannelDataDataspace,
            sys::phaseh5_error_GET_CHANNEL_DATA_DIMS_FAIL => Error::GetChannelDataDims,
            sys::phaseh5_error_NO_RAW_DATA_STREAM => Error::NoRawDataStream,
            sys::phaseh5_error_RAW_DATA_END_BEFORE_START => Error::RawDataEndBeforeStart,
            sys::phaseh5_error_RAW_DATA_END_OUT_OF_BOUNDS => Error::RawDataEndOutOfBounds,
            sys::phaseh5_error_RAW_DATA_GET_DATASPACE_FAIL => Error::RawDataGetDataspace,
            sys::phaseh5_error_RAW_DATA_SELECT_HYPERSLAB_FAIL => Error::RawDataSelectHyperslab,
            sys::phaseh5_error_RAW_DATA_CREATE_MEMORY_DATASPACE_FAIL => Error::RawDataCreateMemoryDataspace,
            sys::phaseh5_error_RAW_DATA_READ_DATA_FAIL => Error::RawDataReadData,
            sys::phaseh5_error_SET_RAW_DATA_GET_DATASPACE_FAIL => Error::SetRawDataGetDataspace,
            sys::phaseh5_error_SET_RAW_DATA_SELECT_HYPERSLAB_FAIL => Error::SetRawDataSelectHyperslab,
            _ => Error::ErrorNotYetConverted,
        }
    }
}

pub struct Phase {
    phase: sys::PhaseH5,
}

impl std::fmt::Debug for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Phase OK")?;
        Ok(())
    }
}

impl std::default::Default for Phase {
    fn default() -> Self {
        Self {
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
            },
        }
    }

}

macro_rules! phase_ptr  {
    ($p:ident) => (&$p.phase as *const sys::PhaseH5 as *mut sys::PhaseH5)
}

impl Phase {
    pub fn open(filename: &str) -> Result<Self, Error> {
        let phase = Self::default();
        let cfilename = CString::new(filename).unwrap();
        let res = unsafe {
           sys::phase_open(
                phase_ptr!(phase),
                cfilename.as_ptr(),
            )
        };
        match res {
            sys::phaseh5_error_OK => Ok(phase),
            _ => Err(Error::from_phaseh5_error(res)),
        }
    }

    pub fn datalen(&self) -> usize {
        return self.phase.datalen;
    }
}

impl Drop for Phase {
    fn drop(&mut self) {
        let _res = unsafe {
            sys::phase_close(phase_ptr!(self));
        };
    }
}

pub fn spike_c_init() {
    unsafe { sys::pycodeh5_init() };
}

pub fn spike_c_close() {
    unsafe { sys::pycodeh5_close() };
}
