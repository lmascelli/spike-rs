use super::*;
use crate::sys::{
    H5Dclose, H5Dget_space, H5Dopen2, H5Dread, H5Fclose, H5Gclose, H5Giterate, H5Oopen, H5Sclose,
    H5Sget_simple_extent_dims, H5Sget_simple_extent_ndims, H5T_C_S1_g, H5T_NATIVE_INT_g,
    H5T_NATIVE_LLONG_g, H5T_class_t_H5T_COMPOUND, H5T_cset_t_H5T_CSET_ASCII,
    H5T_str_t_H5T_STR_NULLPAD, H5Tclose, H5Tcopy, H5Tcreate, H5Tinsert, H5Tset_cset, H5Tset_size,
    H5Tset_strpad, H5open, H5S_ALL,
};
use std::{collections::HashMap, ops::Index};

// thanks to shurizzle
// https://github.com/users/shurizzle
macro_rules! offset_of {
    ($t:ty, $field:ident) => {
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(deref_nullptr)]
            let ptr = &*(0 as *const $t);
            core::mem::transmute::<_, usize>(&ptr.$field)
        }
    };
}

#[repr(C)]
#[derive(Clone)]
struct InfoChannelStruct {
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

macro_rules! print_c_str {
    ($ret:expr, $msg:expr, $var:expr) => {
        if $var as usize != 0 {
            $ret.push_str(&format!(
                "{}:{}\n",
                $msg,
                CString::from_raw($var as *mut c_char)
                    .into_string()
                    .unwrap()
            ));
        } else {
            $ret.push_str(&format!("{}.\n", $msg));
        }
    };
}

impl InfoChannelStruct {
    pub fn info(&self) -> String {
        let mut ret = String::new();
        unsafe {
            ret.push_str(&format!("Channel ID:{}\n", self.channel_id));
            ret.push_str(&format!("Row Index:{}\n", self.row_index));
            ret.push_str(&format!("Group ID:{}\n", self.group_id));
            ret.push_str(&format!("Electrode Group:{}\n", self.electrode_group));
            // print_c_str!(&mut ret, "Label", self.label);
            print_c_str!(&mut ret, "Raw data type:", self.raw_data_type);
            print_c_str!(&mut ret, "Unit:", self.unit);
            ret.push_str(&format!("Exponent:{}\n", self.exponent));
            ret.push_str(&format!("AD Zero:{}\n", self.ad_zero));
            ret.push_str(&format!("Tick:{}\n", self.tick));
            ret.push_str(&format!("Conversion factor:{}\n", self.conversion_factor));
            ret.push_str(&format!("ADC bits:{}\n", self.adc_bits));
            print_c_str!(
                &mut ret,
                "HighPass Filter Type:",
                self.high_pass_filter_type
            );
            // print_c_str!(
            //     &mut ret,
            //     "HighPass Filter CutOff:",
            //     self.high_pass_filter_cutoff
            // );
            ret.push_str(&format!(
                "HighPass Filter Order:{}\n",
                self.high_pass_filter_order
            ));
            print_c_str!(&mut ret, "LowPass Filter Type:", self.low_pass_filter_type);
            // print_c_str!(
            //     &mut ret,
            //     "LowPass Filter CutOff:",
            //     self.low_pass_filter_cutoff
            // );
            ret.push_str(&format!(
                "LowPass Filter Order:{}\n",
                self.low_pass_filter_order
            ));
        }
        return ret;
    }
}

impl InfoChannelStruct {
    fn new() -> InfoChannelStruct {
        InfoChannelStruct {
            channel_id: 0,
            row_index: 0,
            group_id: 0,
            electrode_group: 0,
            label: cstr!("").as_ptr(),
            raw_data_type: cstr!("").as_ptr(),
            unit: cstr!("").as_ptr(),
            exponent: 0,
            ad_zero: 0,
            tick: 0,
            conversion_factor: 0,
            adc_bits: 0,
            high_pass_filter_type: cstr!("").as_ptr(),
            high_pass_filter_cutoff: cstr!("").as_ptr(),
            high_pass_filter_order: 0,
            low_pass_filter_type: cstr!("").as_ptr(),
            low_pass_filter_cutoff: cstr!("").as_ptr(),
            low_pass_filter_order: 0,
        }
    }
}

pub struct H5Analog {
    _group_id: hid_t,
    _name: String,
    channel_data: hid_t,
    n_channels: i64,
    pub labels_dict: HashMap<String, usize>,
    info_channels: Vec<InfoChannelStruct>,
}

impl H5Analog {
    pub fn new(group_id: hid_t, name: &str) -> Option<H5Analog> {
        let mut channel_data = 0;
        let n_channels: i64 = 0;
        let mut info_channels: Vec<InfoChannelStruct> = Vec::new();
        let mut ret;
        unsafe {
            // ChannelData

            channel_data = H5Dopen2(group_id, cstr!("ChannelData").as_ptr(), H5P_DEFAULT.into());
            if channel_data <= 0 {
                return None;
            }

            // InfoChannel

            let info_channel =
                H5Dopen2(group_id, cstr!("InfoChannel").as_ptr(), H5P_DEFAULT.into());

            if info_channel <= 0 {
                return None;
            }

            let info_dataspace = H5Dget_space(info_channel);

            if info_dataspace <= 0 {
                return None;
            }

            H5Sget_simple_extent_dims(
                info_dataspace,
                &n_channels as *const _ as *mut u64,
                0 as *mut u64,
            );

            info_channels.resize(n_channels as usize, InfoChannelStruct::new());

            let info_channel_mem_daatatype = load_info_type();

            H5Dread(
                info_channel,
                info_channel_mem_daatatype,
                H5S_ALL as _,
                H5S_ALL as _,
                H5P_DEFAULT as _,
                info_channels.as_mut_ptr().cast(),
            );

            H5Tclose(info_channel_mem_daatatype);
            ret = H5Analog {
                _group_id: group_id,
                _name: String::from(name),
                channel_data,
                n_channels,
                labels_dict: HashMap::new(),
                info_channels,
            };

            for (i, val) in (ret.info_channels).iter().enumerate() {
                ret.labels_dict.insert(
                    CString::from_raw(val.label as *mut i8)
                        .into_string()
                        .unwrap(),
                    i,
                );
                // println!("{}", ic.info());
            }

            // for (key, value) in &ret.labels_dict {
            //     println!("{key}: {value}");
            // }

            H5Sclose(info_dataspace);
            H5Dclose(info_channel);
        }

        Some(ret)
    }

    pub fn get_channel_data(&self, label: &str) -> Option<Vec<f32>> {
        if let Some(index) = self.labels_dict.get(label) {
            let ic = &self.info_channels[*index];
            let channel_data_index = ic.row_index;
            let ad_zero = ic.ad_zero;
            let conversion_factor = ic.conversion_factor as f32 * 10f32.powf(ic.exponent as f32);
            let ret = Vec::new();

            unsafe {
                let channel_data_dataspace = H5Dget_space(self.channel_data);

                if channel_data_dataspace <= 0 {
                    return None;
                }

                let n_dims = H5Sget_simple_extent_ndims(channel_data_dataspace);
                let mut dims = Vec::new();
                dims.resize(n_dims as usize, 0i64);
                H5Sget_simple_extent_dims(channel_data_dataspace, dims.as_ptr() as _, NULL as _);

                if !(n_dims == 2 && dims[0] == self.n_channels) {
                    return None;
                }

                let storage_slab_start = [0i64];
                let storate_slab_count = [dims[1]];


            }

            Some(ret)
        } else {
            None
        }
    }
}

// impl Index<usize> for H5Analog {
//     type Output = usize ;

//     fn index(&self, i: usize) -> usize {
// 	0
//     }
// }

pub struct H5Content {
    pub analogs: Vec<H5Analog>,
    pub file_id: hid_t,
    pub root_id: hid_t,
}

unsafe extern "C" fn load_analogs(group_id: hid_t, name: *const c_char, data: *mut c_void) -> i32 {
    let analogs = data as *mut Vec<H5Analog>;
    let name = CStr::from_ptr(name).to_str().unwrap();
    let stream_group = H5Oopen(group_id, name.as_ptr() as _, H5P_DEFAULT.into());
    (*analogs).push(H5Analog::new(stream_group, name).unwrap());
    0
}

fn load_info_type() -> hid_t {
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
        info_type_id = H5Tcreate(
            H5T_class_t_H5T_COMPOUND,
            std::mem::size_of::<InfoChannelStruct>(),
        );
        H5Tinsert(
            info_type_id,
            cstr!("ChannelID").as_ptr(),
            offset_of!(InfoChannelStruct, channel_id),
            H5T_NATIVE_INT_g,
        );
        H5Tinsert(
            info_type_id,
            cstr!("RowIndex").as_ptr(),
            offset_of!(InfoChannelStruct, row_index),
            H5T_NATIVE_INT_g,
        );
        H5Tinsert(
            info_type_id,
            cstr!("GroupID").as_ptr(),
            offset_of!(InfoChannelStruct, group_id),
            H5T_NATIVE_INT_g,
        );
        H5Tinsert(
            info_type_id,
            cstr!("ElectrodeGroup").as_ptr(),
            offset_of!(InfoChannelStruct, electrode_group),
            H5T_NATIVE_INT_g,
        );
        H5Tinsert(
            info_type_id,
            cstr!("Label").as_ptr(),
            offset_of!(InfoChannelStruct, label),
            string_type_id,
        );
        H5Tinsert(
            info_type_id,
            cstr!("RawDataType").as_ptr(),
            offset_of!(InfoChannelStruct, raw_data_type),
            string_type_id,
        );
        H5Tinsert(
            info_type_id,
            cstr!("Unit").as_ptr(),
            offset_of!(InfoChannelStruct, unit),
            string_type_id,
        );
        H5Tinsert(
            info_type_id,
            cstr!("Exponent").as_ptr(),
            offset_of!(InfoChannelStruct, exponent),
            H5T_NATIVE_INT_g,
        );
        H5Tinsert(
            info_type_id,
            cstr!("AdZero").as_ptr(),
            offset_of!(InfoChannelStruct, ad_zero),
            H5T_NATIVE_INT_g,
        );
        H5Tinsert(
            info_type_id,
            cstr!("Tick").as_ptr(),
            offset_of!(InfoChannelStruct, tick),
            H5T_NATIVE_LLONG_g,
        );
        H5Tinsert(
            info_type_id,
            cstr!("ConversionFactor").as_ptr(),
            offset_of!(InfoChannelStruct, conversion_factor),
            H5T_NATIVE_LLONG_g,
        );
        H5Tinsert(
            info_type_id,
            cstr!("ADCBits").as_ptr(),
            offset_of!(InfoChannelStruct, adc_bits),
            H5T_NATIVE_INT_g,
        );
        H5Tinsert(
            info_type_id,
            cstr!("HighPassFilterType").as_ptr(),
            offset_of!(InfoChannelStruct, high_pass_filter_type),
            string_type_id,
        );
        H5Tinsert(
            info_type_id,
            cstr!("HighPassFilterCutOff").as_ptr(),
            offset_of!(InfoChannelStruct, high_pass_filter_cutoff),
            string_type_id,
        );
        H5Tinsert(
            info_type_id,
            cstr!("HighPassFilterOrder").as_ptr(),
            offset_of!(InfoChannelStruct, high_pass_filter_order),
            H5T_NATIVE_INT_g,
        );
        H5Tinsert(
            info_type_id,
            cstr!("LowPassFilterType").as_ptr(),
            offset_of!(InfoChannelStruct, low_pass_filter_type),
            string_type_id,
        );
        H5Tinsert(
            info_type_id,
            cstr!("LowPassFilterCutOff").as_ptr(),
            offset_of!(InfoChannelStruct, low_pass_filter_cutoff),
            string_type_id,
        );
        H5Tinsert(
            info_type_id,
            cstr!("LowPassFilterOrder").as_ptr(),
            offset_of!(InfoChannelStruct, low_pass_filter_order),
            H5T_NATIVE_INT_g,
        );
        H5Tclose(string_type_id);
    }
    info_type_id
}

impl H5Content {
    pub fn open(filename: &str) -> Option<H5Content> {
        let file_id = h5open_file(filename, false);
        if let Some(file_id) = file_id {
            let root_id = h5open_group(file_id, "Data/Recording_0");
            if let Some(root_id) = root_id {
                let content = H5Content {
                    analogs: Vec::new(),
                    root_id,
                    file_id,
                };
                unsafe {
                    H5Giterate(
                        content.root_id,
                        cstr!("AnalogStream").as_ptr(),
                        0 as *mut i32,
                        Some(load_analogs),
                        &content.analogs as *const _ as *mut c_void,
                    );
                }
                Some(content)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Drop for H5Content {
    fn drop(&mut self) {
        unsafe {
            H5Gclose(self.root_id);
            H5Fclose(self.file_id);
        }
    }
}
