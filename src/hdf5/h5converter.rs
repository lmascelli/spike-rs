use crate::sys::{
    H5Dclose, H5Dget_space, H5Dopen2, H5Dread, H5Fclose, H5Gclose, H5Giterate, H5Oopen, H5Sclose,
    H5Sget_simple_extent_dims, H5Sget_simple_extent_ndims, H5Tclose, H5S_ALL,
};
use cstr::cstr;
use std::{
    collections::HashMap,
    ffi::{c_char, c_void, CStr},
    fmt, ptr,
};

// thanks to shurizzle
// https://github.com/shurizzle
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

mod untouchable {
    #![allow(dead_code)]

    use crate::sys::{
        hid_t, H5T_C_S1_g, H5T_NATIVE_INT_g, H5T_NATIVE_LLONG_g, H5T_class_t_H5T_COMPOUND,
        H5T_cset_t_H5T_CSET_ASCII, H5T_str_t_H5T_STR_NULLPAD, H5Tclose, H5Tcopy, H5Tcreate,
        H5Tinsert, H5Tset_cset, H5Tset_size, H5Tset_strpad, H5open,
    };
    use core::{
        ffi::{c_char, CStr},
        ptr,
    };
    use cstr::cstr;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct CInfoChannel {
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

    impl CInfoChannel {
        pub fn new() -> CInfoChannel {
            CInfoChannel {
                channel_id: 0,
                row_index: 0,
                group_id: 0,
                electrode_group: 0,
                label: ptr::null_mut(),
                raw_data_type: ptr::null_mut(),
                unit: ptr::null_mut(),
                exponent: 0,
                ad_zero: 0,
                tick: 0,
                conversion_factor: 0,
                adc_bits: 0,
                high_pass_filter_type: ptr::null_mut(),
                high_pass_filter_cutoff: ptr::null_mut(),
                high_pass_filter_order: 0,
                low_pass_filter_type: ptr::null_mut(),
                low_pass_filter_cutoff: ptr::null_mut(),
                low_pass_filter_order: 0,
            }
        }

        #[inline(always)]
        pub fn channel_id(&self) -> i32 {
            self.channel_id
        }

        #[inline(always)]
        pub fn set_channel_id(&mut self, channel_id: i32) {
            self.channel_id = channel_id;
        }

        #[inline(always)]
        pub fn row_index(&self) -> i32 {
            self.row_index
        }

        #[inline(always)]
        pub fn set_row_index(&mut self, row_index: i32) {
            self.row_index = row_index;
        }

        #[inline(always)]
        pub fn group_id(&self) -> i32 {
            self.group_id
        }

        #[inline(always)]
        pub fn set_group_id(&mut self, group_id: i32) {
            self.group_id = group_id;
        }

        #[inline(always)]
        pub fn electrode_group(&self) -> i32 {
            self.electrode_group
        }

        #[inline(always)]
        pub fn set_electrode_group(&mut self, electrode_group: i32) {
            self.electrode_group = electrode_group;
        }

        #[inline(always)]
        pub fn exponent(&self) -> i32 {
            self.exponent
        }

        #[inline(always)]
        pub fn set_exponent(&mut self, exponent: i32) {
            self.exponent = exponent;
        }

        #[inline(always)]
        pub fn ad_zero(&self) -> i32 {
            self.ad_zero
        }

        #[inline(always)]
        pub fn set_ad_zero(&mut self, ad_zero: i32) {
            self.ad_zero = ad_zero;
        }

        #[inline(always)]
        pub fn tick(&self) -> i64 {
            self.tick
        }

        #[inline(always)]
        pub fn set_tick(&mut self, tick: i64) {
            self.tick = tick;
        }

        #[inline(always)]
        pub fn conversion_factor(&self) -> i64 {
            self.conversion_factor
        }

        #[inline(always)]
        pub fn set_conversion_factor(&mut self, conversion_factor: i64) {
            self.conversion_factor = conversion_factor;
        }

        #[inline(always)]
        pub fn adc_bits(&self) -> i32 {
            self.adc_bits
        }

        #[inline(always)]
        pub fn set_adc_bits(&mut self, adc_bits: i32) {
            self.adc_bits = adc_bits;
        }

        #[inline(always)]
        pub fn high_pass_filter_order(&self) -> i32 {
            self.high_pass_filter_order
        }

        #[inline(always)]
        pub fn set_high_pass_filter_order(&mut self, high_pass_filter_order: i32) {
            self.high_pass_filter_order = high_pass_filter_order;
        }

        #[inline(always)]
        pub fn low_pass_filter_order(&self) -> i32 {
            self.low_pass_filter_order
        }

        #[inline(always)]
        pub fn set_low_pass_filter_order(&mut self, low_pass_filter_order: i32) {
            self.low_pass_filter_order = low_pass_filter_order;
        }

        #[inline(always)]
        pub fn label(&self) -> Option<&CStr> {
            ocstr(self.label)
        }

        #[inline(always)]
        pub fn raw_data_type(&self) -> Option<&CStr> {
            ocstr(self.raw_data_type)
        }

        #[inline(always)]
        pub fn unit(&self) -> Option<&CStr> {
            ocstr(self.unit)
        }

        #[inline(always)]
        pub fn high_pass_filter_type(&self) -> Option<&CStr> {
            ocstr(self.high_pass_filter_type)
        }

        #[inline(always)]
        pub fn high_pass_filter_cutoff(&self) -> Option<&CStr> {
            ocstr(self.high_pass_filter_cutoff)
        }

        #[inline(always)]
        pub fn low_pass_filter_type(&self) -> Option<&CStr> {
            ocstr(self.low_pass_filter_type)
        }

        #[inline(always)]
        pub fn low_pass_filter_cutoff(&self) -> Option<&CStr> {
            ocstr(self.low_pass_filter_cutoff)
        }

        #[inline(always)]
        pub fn set_label(&mut self, value: Option<&'static CStr>) {
            self.label = ocstr_ptr(value);
        }

        #[inline(always)]
        pub fn set_raw_data_type(&mut self, value: Option<&'static CStr>) {
            self.raw_data_type = ocstr_ptr(value);
        }

        #[inline(always)]
        pub fn set_unit(&mut self, value: Option<&'static CStr>) {
            self.unit = ocstr_ptr(value);
        }

        #[inline(always)]
        pub fn set_high_pass_filter_type(&mut self, value: Option<&'static CStr>) {
            self.high_pass_filter_type = ocstr_ptr(value);
        }

        #[inline(always)]
        pub fn set_high_pass_filter_cutoff(&mut self, value: Option<&'static CStr>) {
            self.high_pass_filter_cutoff = ocstr_ptr(value);
        }

        #[inline(always)]
        pub fn set_low_pass_filter_type(&mut self, value: Option<&'static CStr>) {
            self.low_pass_filter_type = ocstr_ptr(value);
        }

        #[inline(always)]
        pub fn set_low_pass_filter_cutoff(&mut self, value: Option<&'static CStr>) {
            self.low_pass_filter_cutoff = ocstr_ptr(value);
        }
    }

    #[inline]
    fn ocstr<'a>(ptr: *const c_char) -> Option<&'a CStr> {
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }

    #[inline]
    fn ocstr_ptr(str: Option<&'static CStr>) -> *const c_char {
        if let Some(str) = str {
            str.as_ptr()
        } else {
            ptr::null()
        }
    }

    pub fn load_info_type() -> hid_t {
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
                std::mem::size_of::<CInfoChannel>(),
            );
            H5Tinsert(
                info_type_id,
                cstr!("ChannelID").as_ptr(),
                offset_of!(CInfoChannel, channel_id),
                H5T_NATIVE_INT_g,
            );
            H5Tinsert(
                info_type_id,
                cstr!("RowIndex").as_ptr(),
                offset_of!(CInfoChannel, row_index),
                H5T_NATIVE_INT_g,
            );
            H5Tinsert(
                info_type_id,
                cstr!("GroupID").as_ptr(),
                offset_of!(CInfoChannel, group_id),
                H5T_NATIVE_INT_g,
            );
            H5Tinsert(
                info_type_id,
                cstr!("ElectrodeGroup").as_ptr(),
                offset_of!(CInfoChannel, electrode_group),
                H5T_NATIVE_INT_g,
            );
            H5Tinsert(
                info_type_id,
                cstr!("Label").as_ptr(),
                offset_of!(CInfoChannel, label),
                string_type_id,
            );
            H5Tinsert(
                info_type_id,
                cstr!("RawDataType").as_ptr(),
                offset_of!(CInfoChannel, raw_data_type),
                string_type_id,
            );
            H5Tinsert(
                info_type_id,
                cstr!("Unit").as_ptr(),
                offset_of!(CInfoChannel, unit),
                string_type_id,
            );
            H5Tinsert(
                info_type_id,
                cstr!("Exponent").as_ptr(),
                offset_of!(CInfoChannel, exponent),
                H5T_NATIVE_INT_g,
            );
            H5Tinsert(
                info_type_id,
                cstr!("AdZero").as_ptr(),
                offset_of!(CInfoChannel, ad_zero),
                H5T_NATIVE_INT_g,
            );
            H5Tinsert(
                info_type_id,
                cstr!("Tick").as_ptr(),
                offset_of!(CInfoChannel, tick),
                H5T_NATIVE_LLONG_g,
            );
            H5Tinsert(
                info_type_id,
                cstr!("ConversionFactor").as_ptr(),
                offset_of!(CInfoChannel, conversion_factor),
                H5T_NATIVE_LLONG_g,
            );
            H5Tinsert(
                info_type_id,
                cstr!("ADCBits").as_ptr(),
                offset_of!(CInfoChannel, adc_bits),
                H5T_NATIVE_INT_g,
            );
            H5Tinsert(
                info_type_id,
                cstr!("HighPassFilterType").as_ptr(),
                offset_of!(CInfoChannel, high_pass_filter_type),
                string_type_id,
            );
            H5Tinsert(
                info_type_id,
                cstr!("HighPassFilterCutOff").as_ptr(),
                offset_of!(CInfoChannel, high_pass_filter_cutoff),
                string_type_id,
            );
            H5Tinsert(
                info_type_id,
                cstr!("HighPassFilterOrder").as_ptr(),
                offset_of!(CInfoChannel, high_pass_filter_order),
                H5T_NATIVE_INT_g,
            );
            H5Tinsert(
                info_type_id,
                cstr!("LowPassFilterType").as_ptr(),
                offset_of!(CInfoChannel, low_pass_filter_type),
                string_type_id,
            );
            H5Tinsert(
                info_type_id,
                cstr!("LowPassFilterCutOff").as_ptr(),
                offset_of!(CInfoChannel, low_pass_filter_cutoff),
                string_type_id,
            );
            H5Tinsert(
                info_type_id,
                cstr!("LowPassFilterOrder").as_ptr(),
                offset_of!(CInfoChannel, low_pass_filter_order),
                H5T_NATIVE_INT_g,
            );
            H5Tclose(string_type_id);
        }
        info_type_id
    }
}
use untouchable::{load_info_type, CInfoChannel};

impl fmt::Debug for CInfoChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InfoChannelStruct")
            .field("channel_id", &self.channel_id())
            .field("row_index", &self.row_index())
            .field("group_id", &self.group_id())
            .field("electrode_group", &self.electrode_group())
            .field("label", &self.label())
            .field("raw_data_type", &self.raw_data_type())
            .field("unit", &self.unit())
            .field("exponent", &self.exponent())
            .field("ad_zero", &self.ad_zero())
            .field("tick", &self.tick())
            .field("conversion_factor", &self.conversion_factor())
            .field("adc_bits", &self.adc_bits())
            .field("high_pass_filter_type", &self.high_pass_filter_type())
            .field("high_pass_filter_cutoff", &self.high_pass_filter_cutoff())
            .field("high_pass_filter_order", &self.high_pass_filter_order())
            .field("low_pass_filter_type", &self.low_pass_filter_type())
            .field("low_pass_filter_cutoff", &self.low_pass_filter_cutoff())
            .field("low_pass_filter_order", &self.low_pass_filter_order())
            .finish()
    }
}

use super::{h5open_file, h5open_group, hid_t, H5P_DEFAULT};

pub struct H5Analog {
    _group_id: hid_t,
    _name: String,
    channel_data: hid_t,
    n_channels: i64,
    pub labels_dict: HashMap<String, usize>,
    info_channels: Vec<CInfoChannel>,
}

impl H5Analog {
    pub fn new(group_id: hid_t, name: &str) -> Option<H5Analog> {
        let channel_data;
        let mut n_channels: i64 = 0;
        let mut info_channels: Vec<CInfoChannel> = Vec::new();
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
                &mut n_channels as *mut i64 as *mut u64,
                ptr::null_mut(),
            );

            info_channels.resize(n_channels as usize, CInfoChannel::new());

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
                    val.label()
                        .map(CStr::to_string_lossy)
                        .unwrap_or("".into())
                        .to_string(),
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
            let channel_data_index = ic.row_index();
            let ad_zero = ic.ad_zero();
            let conversion_factor =
                ic.conversion_factor() as f32 * 10f32.powf(ic.exponent() as f32);
            let ret = Vec::new();

            unsafe {
                let channel_data_dataspace = H5Dget_space(self.channel_data);

                if channel_data_dataspace <= 0 {
                    return None;
                }

                let n_dims = H5Sget_simple_extent_ndims(channel_data_dataspace);
                let mut dims = vec![0i64; n_dims as usize];
                H5Sget_simple_extent_dims(
                    channel_data_dataspace,
                    dims.as_mut_ptr().cast(),
                    ptr::null_mut(),
                );

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

impl H5Content {
    pub fn open(filename: &str) -> Option<H5Content> {
        let file_id = h5open_file(filename, false);
        if let Some(file_id) = file_id {
            let root_id = h5open_group(file_id, "Data/Recording_0");
            if let Some(root_id) = root_id {
                let mut analogs = Vec::new();
                unsafe {
                    H5Giterate(
                        root_id,
                        cstr!("AnalogStream").as_ptr(),
                        ptr::null_mut(),
                        Some(load_analogs),
                        &mut analogs as *mut _ as *mut c_void,
                    );
                }
                Some(H5Content {
                    analogs,
                    file_id,
                    root_id,
                })
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

