use hdf5_rs::h5sys::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CInfoChannel {
    pub channel_id: i32,
    pub row_index: i32,
    pub group_id: i32,
    pub electrode_group: i32,
    pub label: *const c_char,
    pub raw_data_type: *const c_char,
    pub unit: *const c_char,
    pub exponent: i32,
    pub ad_zero: i32,
    pub tick: i64,
    pub conversion_factor: i64,
    pub adc_bits: i32,
    pub high_pass_filter_type: *const c_char,
    pub high_pass_filter_cutoff: *const c_char,
    pub high_pass_filter_order: i32,
    pub low_pass_filter_type: *const c_char,
    pub low_pass_filter_cutoff: *const c_char,
    pub low_pass_filter_order: i32,
}

impl Default for CInfoChannel {
    fn default() -> Self {
        CInfoChannel {
            channel_id: 0i32,
            row_index: 0i32,
            group_id: 0i32,
            electrode_group: 0i32,
            label: null(),
            raw_data_type: null(),
            unit: null(),
            exponent: 0i32,
            ad_zero: 0i32,
            tick: 0i64,
            conversion_factor: 0i64,
            adc_bits: 0i32,
            high_pass_filter_type: null(),
            high_pass_filter_cutoff: null(),
            high_pass_filter_order: 0i32,
            low_pass_filter_type: null(),
            low_pass_filter_cutoff: null(),
            low_pass_filter_order: 0i32,
        }
    }
}

macro_rules! offset_of {
    ($typename:ty, $field:ident) => {
        &(*(0 as *const $typename)).$field as *const _ as usize
    };
}

#[allow(deref_nullptr, unused)]
pub fn info_channel_type() -> types::Hid {
    let string_type_id;
    let info_type_id;
    unsafe {
        string_type_id = datatype::H5Tcopy({
            lib::H5open();
            datatype::H5T_C_S1_g
        });
        datatype::H5Tset_size(string_type_id, usize::MAX);
        datatype::H5Tset_strpad(string_type_id, datatype::H5T_str_t_H5T_STR_NULLPAD);
        datatype::H5Tset_cset(string_type_id, datatype::H5T_cset_t_H5T_CSET_ASCII);

        info_type_id = datatype::H5Tcreate(
            datatype::H5T_class_t_H5T_COMPOUND,
            std::mem::size_of::<CInfoChannel>(),
        );

        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("ChannelID\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, channel_id),
            datatype::H5T_NATIVE_INT_g,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("RowIndex\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, row_index),
            datatype::H5T_NATIVE_INT_g,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("GroupId\0".as_bytes()).unwrap().as_ptr(),
            offset_of!(CInfoChannel, group_id),
            datatype::H5T_NATIVE_INT_g,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("ElectrodeGroup\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, electrode_group),
            datatype::H5T_NATIVE_INT_g,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("Label\0".as_bytes()).unwrap().as_ptr(),
            offset_of!(CInfoChannel, label),
            string_type_id,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("RawDataType\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, raw_data_type),
            string_type_id,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("Unit\0".as_bytes()).unwrap().as_ptr(),
            offset_of!(CInfoChannel, unit),
            string_type_id,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("Exponent\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, exponent),
            datatype::H5T_NATIVE_INT_g,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("AdZero\0".as_bytes()).unwrap().as_ptr(),
            offset_of!(CInfoChannel, ad_zero),
            datatype::H5T_NATIVE_INT_g,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("Tick\0".as_bytes()).unwrap().as_ptr(),
            offset_of!(CInfoChannel, tick),
            datatype::H5T_NATIVE_LLONG_g,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("ConversionFactor\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, conversion_factor),
            datatype::H5T_NATIVE_LLONG_g,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("ADCBits\0".as_bytes()).unwrap().as_ptr(),
            offset_of!(CInfoChannel, adc_bits),
            datatype::H5T_NATIVE_INT_g,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("HighPassFilterType\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, high_pass_filter_type),
            string_type_id,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("HighPassFilterCutOff\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, high_pass_filter_cutoff),
            string_type_id,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("HighPassFilterOrder\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, high_pass_filter_order),
            datatype::H5T_NATIVE_INT_g,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("LowPassFilterType\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, low_pass_filter_type),
            string_type_id,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("LowPassFilterCutOff\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, low_pass_filter_cutoff),
            string_type_id,
        );
        datatype::H5Tinsert(
            info_type_id,
            CStr::from_bytes_with_nul("LowPassFilterOrder\0".as_bytes())
                .unwrap()
                .as_ptr(),
            offset_of!(CInfoChannel, low_pass_filter_order),
            datatype::H5T_NATIVE_INT_g,
        );
        datatype::H5Tclose(string_type_id);
    }
    info_type_id
}
