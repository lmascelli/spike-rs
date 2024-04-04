pub mod sys {
    #![allow(unused)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(clippy::upper_case_acronyms)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use std::ffi::{c_char, c_void, CStr, CString};
pub use std::ptr::{null, null_mut};

pub use sys::{
    H5A_info_t, H5Aclose, H5Aget_space, H5Aget_type, H5Aiterate2, H5Aopen,
    H5Aread,

    H5Dclose, H5Dcreate2, H5Dget_space, H5Dget_type, H5Dopen2, H5Dread, H5Dwrite,

    H5Fclose, H5Fcreate, H5Fopen,

    H5Gclose, H5Gcreate2, H5Gopen2,

    H5Iget_name,

    H5L_info2_t, H5Literate2,

    H5Pcopy,

    H5Sclose, H5Screate_simple, H5Sget_simple_extent_dims,
    H5Sget_simple_extent_ndims, H5Sselect_all, H5Sselect_hyperslab, H5S_class_t_H5S_SCALAR,
    H5Screate, H5S_seloper_t_H5S_SELECT_SET,

    H5T_class_t_H5T_STRING, H5T_class_t_H5T_INTEGER, H5T_class_t_H5T_COMPOUND,
    H5T_class_t_H5T_FLOAT,

    H5T_sign_t_H5T_SGN_NONE, H5T_sign_t_H5T_SGN_2,

    H5T_str_t_H5T_STR_NULLPAD, H5T_str_t_H5T_STR_NULLTERM, H5T_cset_t_H5T_CSET_ASCII,

    H5T_C_S1_g, H5T_NATIVE_FLOAT_g, H5T_NATIVE_INT_g, H5T_NATIVE_LLONG_g,
    H5T_NATIVE_ULLONG_g,

    H5Tclose, H5Tcopy, H5Tcreate, H5Tget_class, H5Tget_cset, H5Tget_sign, H5Tget_size,
    H5Tget_strpad, H5Tinsert, H5Tset_cset, H5Tis_variable_str, H5Tset_size, H5Tset_strpad,

    H5_index_t_H5_INDEX_NAME, H5_iter_order_t_H5_ITER_INC, H5open,
};

pub const H5F_ACC_RDONLY: u32 = 0;
pub const H5F_ACC_TRUNC: u32 = 2;
pub const H5P_DEFAULT: i64 = sys::H5P_DEFAULT as i64;
pub const H5S_ALL: i64 = sys::H5S_ALL as i64;
pub const H5S_SELECT_SET: i32 = sys::H5S_seloper_t_H5S_SELECT_SET;