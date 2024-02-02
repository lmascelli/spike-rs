mod sys {
  #![allow(unused)]
  #![allow(non_upper_case_globals)]
  #![allow(non_camel_case_types)]
  #![allow(non_snake_case)]
  #![allow(clippy::upper_case_acronyms)]
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr::{null, null_mut};

use crate::core::types::{Phase, Signal};

use sys::{
    H5Dclose, H5Dcreate2, H5Dget_space, H5Dopen2, H5Dread, H5Dwrite, H5Fclose, H5Fcreate, H5Fopen,
    H5Gclose, H5Gcreate2, H5Gopen2, H5L_info2_t, H5Literate2, H5Sclose, H5Screate_simple,
    H5Sget_simple_extent_dims, H5Sget_simple_extent_ndims, H5Sselect_hyperslab, H5T_C_S1_g,
    H5T_NATIVE_FLOAT_g, H5T_NATIVE_INT_g, H5T_NATIVE_LLONG_g, H5T_NATIVE_ULLONG_g,
    H5T_class_t_H5T_COMPOUND, H5T_cset_t_H5T_CSET_ASCII, H5T_str_t_H5T_STR_NULLPAD, H5Tclose,
    H5Tcopy, H5Tcreate, H5Tinsert, H5Tset_cset, H5Tset_size, H5Tset_strpad,
    H5_index_t_H5_INDEX_NAME, H5_iter_order_t_H5_ITER_INC, H5open, H5S_class_t_H5S_SCALAR, H5Screate,
};

const H5F_ACC_RDONLY: u32 = 0;                                                                     
const H5F_ACC_TRUNC: u32 = 2;
const H5P_DEFAULT: i64 = sys::H5P_DEFAULT as i64;
const H5S_ALL: i64 = sys::H5S_ALL as i64;
const H5S_SELECT_SET: i32 = sys::H5S_seloper_t_H5S_SELECT_SET;



////////////////////////////////////////////////////////////////////////////////
///
///                                   Utils
///
////////////////////////////////////////////////////////////////////////////////

extern "C" fn _print_group_name(_group: i64,
                                name:   *const i8,
                                _info:  *const H5L_info2_t,
                                _data:  *mut c_void,
                                ) -> i32 {
    let name = unsafe { CStr::from_ptr(name).to_str().unwrap() };
    println!("{name}");
    0
}

#[allow(unused)]
pub fn print_group_names(group: i64) {
    unsafe {
        H5Literate2(group,
                    H5_index_t_H5_INDEX_NAME,
                    H5_iter_order_t_H5_ITER_INC,
                    null_mut(),
                    Some(_print_group_name),
                    null_mut());
    }
}

////////////////////////////////////////////////////////////////////////////////
///
///                                   Phase Saver
///
////////////////////////////////////////////////////////////////////////////////

pub fn save_phase(phase: &Phase, filename: &str) -> Result<(), String> {
    if let Ok(cfilename) = CString::new(filename) {
        let mut sampling_frequency = 0f32;
        let savefile_id = unsafe { H5Fcreate(cfilename.as_c_str().as_ptr(), H5F_ACC_TRUNC,
        H5P_DEFAULT, H5P_DEFAULT) };
        if savefile_id > 0 {

            // save digitals
            for (i, digital) in phase.digitals.iter().enumerate() {
                let digital_name = format!("digital_{i}\0");
                let digital_len = vec![digital.data.len() as u64];
                let digital_dataspace = unsafe {
                    H5Screate_simple(1, digital_len.as_ptr() ,null())
                };
                let digital_dataset = unsafe {H5Dcreate2(savefile_id,
                                                         CStr::from_bytes_with_nul(digital_name
                                                                                   .as_str()
                                                                                   .as_bytes())
                                                         .unwrap().as_ptr(),
                                                         H5T_NATIVE_FLOAT_g,
                                                         digital_dataspace,
                                                         H5P_DEFAULT,
                                                         H5P_DEFAULT,
                                                         H5P_DEFAULT)};
                if digital_dataset > 0 {
                    unsafe { 
                        H5Dwrite(digital_dataset,
                                 H5T_NATIVE_FLOAT_g,
                                 digital_dataspace,
                                 H5S_ALL,
                                 H5P_DEFAULT,
                                 digital.data.as_ptr().cast());
                        H5Dclose(digital_dataset);
                    }
                } else {
                    return Err(format!("save_phase: failed to create digital group {}", digital_name));
                }
            }

            // save raw_datas
            let raw_data_name = format!("raw_data\0");
            let raw_data_group = unsafe {H5Gcreate2(savefile_id,
                                                    CStr::from_bytes_with_nul(raw_data_name
                                                                              .as_str()
                                                                              .as_bytes())
                                                    .unwrap().as_ptr(),
                                                    H5P_DEFAULT,
                                                    H5P_DEFAULT,
                                                    H5P_DEFAULT) };
            if raw_data_group > 0 {
                for (label, channel) in &phase.raw_data {
                    sampling_frequency = channel.sampling_frequency;
                    let channel_name = format!("{label}\0");
                    let channel_len = vec![channel.data.len() as u64];
                    let channel_dataspace = unsafe {
                        H5Screate_simple(1, channel_len.as_ptr() ,null())
                    };
                    let channel_dataset = unsafe {H5Dcreate2(raw_data_group,
                                                             CStr::from_bytes_with_nul(channel_name
                                                                                       .as_str()
                                                                                       .as_bytes())
                                                             .unwrap().as_ptr(),
                                                             H5T_NATIVE_FLOAT_g,
                                                             channel_dataspace,
                                                             H5P_DEFAULT,
                                                             H5P_DEFAULT,
                                                             H5P_DEFAULT)};

                    if channel_dataset > 0 {
                        unsafe { 
                            H5Dwrite(channel_dataset,
                                     H5T_NATIVE_FLOAT_g,
                                     channel_dataspace,
                                     H5S_ALL,
                                     H5P_DEFAULT,
                                     channel.data.as_ptr().cast());
                            H5Dclose(channel_dataset);
                        }
                    } else {
                        return Err(format!("save_phase: failed to create raw_data group {}", label));
                    }
                }
                unsafe { H5Gclose(raw_data_group) };
            } else {
                return Err(format!("save_phase: failed to create raw_data group {}", raw_data_name));
            }

            // save the sampling frequency TODO
            let sampling_frequency_dataspace = unsafe { H5Screate(H5S_class_t_H5S_SCALAR) };
            let sampling_frequency_dataset = unsafe { H5Dcreate2(savefile_id,
                                                                 CStr::from_bytes_with_nul("sampling_frequency\0".as_bytes())
                                                                 .unwrap().as_ptr(),
                                                                 H5T_NATIVE_FLOAT_g,
                                                                 sampling_frequency_dataspace,
                                                                 H5P_DEFAULT,
                                                                 H5P_DEFAULT,
                                                                 H5P_DEFAULT) };
            unsafe {
                H5Dwrite(sampling_frequency_dataset,
                         H5T_NATIVE_FLOAT_g,
                         sampling_frequency_dataspace,
                         H5S_ALL,
                         H5P_DEFAULT,
                         (&sampling_frequency as *const f32).cast());
            }

            // save peak_trains
            let peaks_train_name = format!("peaks_train\0");
            let peaks_train_group = unsafe {H5Gcreate2(savefile_id,
                                                       CStr::from_bytes_with_nul(peaks_train_name
                                                                                 .as_str()
                                                                                 .as_bytes())
                                                       .unwrap().as_ptr(),
                                                       H5P_DEFAULT,
                                                       H5P_DEFAULT,
                                                       H5P_DEFAULT) };
            if peaks_train_group > 0 {
                for (label, channel) in &phase.peaks_trains {
                    let channel_name = format!("{label}\0");
                    let channel_len = vec![channel.len() as u64];
                    let channel_dataspace = unsafe {
                        H5Screate_simple(1, channel_len.as_ptr() ,null())
                    };
                    let channel_dataset = unsafe {H5Dcreate2(raw_data_group,
                                                             CStr::from_bytes_with_nul(channel_name
                                                                                       .as_str()
                                                                                       .as_bytes())
                                                             .unwrap().as_ptr(),
                                                             H5T_NATIVE_FLOAT_g,
                                                             channel_dataspace,
                                                             H5P_DEFAULT,
                                                             H5P_DEFAULT,
                                                             H5P_DEFAULT)};

                    if channel_dataset > 0 {
                        unsafe { 
                            H5Dwrite(channel_dataset,
                                     H5T_NATIVE_ULLONG_g,
                                     channel_dataspace,
                                     H5S_ALL,
                                     H5P_DEFAULT,
                                     channel.as_ptr().cast());
                            H5Dclose(channel_dataset);
                        }
                    } else {
                        return Err(format!("save_phase: failed to create peaks_train group {}", label));
                    }
                }
                unsafe { H5Gclose(peaks_train_group) };
            } else {
                return Err(format!("save_phase: failed to create peaks_train group {}", peaks_train_name));
            }

            unsafe { H5Fclose(savefile_id) };
            Ok(())
        } else {
            Err(format!("save_phase: failed to create file {}", filename))
        }
    } else {
        Err(format!("save_phase: invalid filename {}", filename))
    }
}

////////////////////////////////////////////////////////////////////////////////
///
///                                   Phase Loader
///
////////////////////////////////////////////////////////////////////////////////

pub fn load_phase(filename: &str) -> Result<Phase, String> {
    let mut ret = Phase::default();
    let cfilename = format!("{filename}\0");
    let file_id = unsafe { H5Fopen(CStr::from_bytes_with_nul(cfilename.as_bytes())
                                   .unwrap().as_ptr(),
                                   H5F_ACC_RDONLY,
                                   H5P_DEFAULT) };

    if file_id > 0 {
        // read sampling frequency
        
        // read digital channels
        
        // read raw_data channels
        
        // read peak_train channels

        unsafe { H5Fclose(file_id); }
        Ok(ret)
    } else {
        Err(format!("load_phase: failed opening file {}", filename))
    }
}


////////////////////////////////////////////////////////////////////////////////
///
///                          MultiChannel Converted
///
////////////////////////////////////////////////////////////////////////////////


#[repr(C)]
#[derive(Clone, Copy)]
struct CInfoChannel {
    channel_id: i32,
    row_index: i32,
    group_id: i32,
    electrode_group: i32,
    label: *const c_char,
    raw_data_type: *const c_char,
    unit: *const c_char,
    exponent: i32,
    ad_zero: i32,
    tick: i64,
    conversion_factor: i64,
    adc_bits: i32,
    high_pass_filter_type: *const c_char,
    high_pass_filter_cutoff: *const c_char,
    high_pass_filter_order: i32,
    low_pass_filter_type: *const c_char,
    low_pass_filter_cutoff: *const c_char,
    low_pass_filter_order: i32,
}

impl Default for CInfoChannel {
    fn default() -> Self {
        CInfoChannel {
            channel_id:                         0i32,
            row_index:                          0i32,
            group_id:                           0i32,
            electrode_group:                    0i32,
            label:                              null(),
            raw_data_type:                      null(),
            unit:                               null(),
            exponent:                           0i32,
            ad_zero:                            0i32,
            tick:                               0i64,
            conversion_factor:                  0i64,
            adc_bits:                           0i32,
            high_pass_filter_type:              null(),
            high_pass_filter_cutoff:            null(),
            high_pass_filter_order:             0i32,
            low_pass_filter_type:               null(),
            low_pass_filter_cutoff:             null(),
            low_pass_filter_order:              0i32,
        }
    }
}

macro_rules! offset_of {
    ($typename:ty, $field:ident) => {
        &(*(0 as *const $typename)).$field as *const _ as usize
    };
}

#[allow(deref_nullptr)]
fn load_info_type() -> i64 {
    let string_type_id;
    let info_type_id;
    unsafe {
        string_type_id = H5Tcopy({
            H5open();
            H5T_C_S1_g
        });
        H5Tset_size(string_type_id, usize::MAX);
        H5Tset_strpad(string_type_id, H5T_str_t_H5T_STR_NULLPAD);
        H5Tset_cset(string_type_id, H5T_cset_t_H5T_CSET_ASCII);

        info_type_id = H5Tcreate(H5T_class_t_H5T_COMPOUND,
                                 std::mem::size_of::<CInfoChannel>(),
                                 );

        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("ChannelID\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, channel_id),
                  H5T_NATIVE_INT_g);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("RowIndex\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, row_index),
                  H5T_NATIVE_INT_g);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("GroupId\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, group_id),
                  H5T_NATIVE_INT_g);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("ElectrodeGroup\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, electrode_group),
                  H5T_NATIVE_INT_g);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("Label\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, label),
                  string_type_id);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("RawDataType\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, raw_data_type),
                  string_type_id);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("Unit\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, unit),
                  string_type_id);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("Exponent\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, exponent),
                  H5T_NATIVE_INT_g);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("AdZero\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, ad_zero),
                  H5T_NATIVE_INT_g);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("Tick\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, tick),
                  H5T_NATIVE_LLONG_g);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("ConversionFactor\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, conversion_factor),
                  H5T_NATIVE_LLONG_g);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("ADCBits\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, adc_bits),
                  H5T_NATIVE_INT_g);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("HighPassFilterType\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, high_pass_filter_type),
                  string_type_id);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("HighPassFilterCutOff\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, high_pass_filter_cutoff),
                  string_type_id);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("HighPassFilterOrder\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, high_pass_filter_order),
                  H5T_NATIVE_INT_g);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("LowPassFilterType\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, low_pass_filter_type),
                  string_type_id);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("LowPassFilterCutOff\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, low_pass_filter_cutoff),
                  string_type_id);
        H5Tinsert(info_type_id,
                  CStr::from_bytes_with_nul("LowPassFilterOrder\0".as_bytes()).unwrap().as_ptr(),
                  offset_of!(CInfoChannel, low_pass_filter_order),
                  H5T_NATIVE_INT_g);
        H5Tclose(string_type_id);
    }
    info_type_id
}

extern "C" fn _parse_analog_stream(group: i64,
                                   name: *const i8,
                                   _info: *const H5L_info2_t,
                                   data: *mut c_void) -> i32 {
    let phase = unsafe { &mut*(data as *mut Phase) };
    let inner_group = unsafe { H5Gopen2(group, name, H5P_DEFAULT) };
    let mut is_digital = false;
    let info_channel_dataset;
    let mut info_channels = Vec::new();
    unsafe {
        info_channel_dataset = H5Dopen2(inner_group,
                                        CStr::from_bytes_with_nul("InfoChannel\0"
                                                                  .as_bytes()).unwrap().as_ptr(),
                                                                  H5P_DEFAULT);
        let info_channel_dataspace = H5Dget_space(info_channel_dataset);

        // get the number of channels
        let mut dims = 0u64;
        H5Sget_simple_extent_dims(info_channel_dataspace, &mut dims as *mut u64, null_mut());
        if dims == 1 {
            is_digital = true;
        }

        // prepare memory for holding infochannels metadata
        info_channels.resize(dims as usize, CInfoChannel::default());
        let info_channel_memory_datatype = load_info_type();

        // read the metadatas
        H5Dread(info_channel_dataset, info_channel_memory_datatype, H5S_ALL, H5S_ALL,
                H5P_DEFAULT, info_channels.as_ptr() as _);

        H5Tclose(info_channel_memory_datatype);
        H5Sclose(info_channel_dataspace);
        H5Dclose(info_channel_dataset);
    }
    // 
    let channel_data_dataset;
    unsafe {
        channel_data_dataset = H5Dopen2(inner_group,
                                        CStr::from_bytes_with_nul("ChannelData\0"
                                                                  .as_bytes()).unwrap().as_ptr(),
                                                                  H5P_DEFAULT);

        let channel_data_dataspace = H5Dget_space(channel_data_dataset);
        let ndims = H5Sget_simple_extent_ndims(channel_data_dataspace);
        let dims = vec![0; ndims as usize];
        H5Sget_simple_extent_dims(channel_data_dataspace, dims.as_ptr().cast_mut(), null_mut());

        // let n_channels = dims[0];
        let n_samples = dims[1];

        for info_channel in info_channels {
            // get channel label
            let label = CStr::from_ptr(info_channel.label).to_str().unwrap();

            let sampling_frequency = 1e4f32;

            // get channel row in dataspace
            let row_index = info_channel.row_index as u64;

            // get channel adc offset
            let adc_offset = info_channel.ad_zero as f32;

            // get channel conversion factor
            let conversion_factor = info_channel.conversion_factor as f32 * f32::powf(10f32, info_channel.exponent as f32);

            // set the dataspace slub
            let starting_point = [row_index, 0];
            let length_data_to_read = [1u64, n_samples];
            H5Sselect_hyperslab(channel_data_dataspace, H5S_SELECT_SET, starting_point.as_ptr(),
            null(), length_data_to_read.as_ptr(), null());

            // allocate the memory;
            let data_to_be_converted = vec![0i32; n_samples as usize];

            // create the memory dataspace
            let memory_size = [dims[1]];
            let channel_data_memory_dataspace = H5Screate_simple(1, memory_size.as_ptr(), null_mut());

            // read the data
            H5Dread(channel_data_dataset, H5T_NATIVE_INT_g, channel_data_memory_dataspace,
                    channel_data_dataspace, H5P_DEFAULT, data_to_be_converted.as_ptr() as _);

            // convert the data
            let mut converted_data = vec![0f32; n_samples as usize];

            for (i, value) in data_to_be_converted.iter().enumerate() {
                converted_data[i] = (*value as f32 - adc_offset) * conversion_factor;
            }

            if is_digital {
                phase.digitals.push(Signal::new(converted_data, sampling_frequency));
            } else {
                phase.raw_data.insert(label.to_string(), Signal::new(converted_data, sampling_frequency));
            }

        }

        H5Sclose(channel_data_dataspace);
        H5Dclose(channel_data_dataset);
    }
    0
}

pub fn convert_mc_h5_file(filename: &str) -> Result<Phase, String> {

    let ret = Phase::default();

    // Open file and get the Recording_0 group
    if let Ok(cfilename) = CString::new(filename) {
        let fid = unsafe { H5Fopen(cfilename.as_c_str().as_ptr(), H5F_ACC_RDONLY, H5P_DEFAULT) };
        if fid <= 0 {
            return Err(format!("convert_mc_h5_file: failed opening {}", filename));
        }
        let analogs_id = unsafe { H5Gopen2(fid,
                                           CStr::from_bytes_with_nul("/Data/Recording_0/AnalogStream\0"
                                                                     .as_bytes()).unwrap().as_ptr(),
                                                                     H5P_DEFAULT) };
        if analogs_id <= 0 {
            return Err(format!("convert_mc_h5_file: error opening Recording_0 group in file {}", filename));
        }

        // parse the Stream_X channels in the analogs_id
        unsafe {
            H5Literate2(analogs_id, 
                        H5_index_t_H5_INDEX_NAME,
                        H5_iter_order_t_H5_ITER_INC,
                        null_mut(),
                        Some(_parse_analog_stream),
                        &ret as *const Phase as *mut c_void);
        }

        unsafe {
            H5Gclose(analogs_id);
            H5Fclose(fid);
        }
        Ok(ret)
    } else {
        Err(format!("convert_mc_h5_file: invalid filename {}", filename))
    }
}
