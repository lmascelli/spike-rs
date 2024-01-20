pub use cstr::cstr;

pub use crate::sys::{
    hid_t, H5Fopen, H5Gopen2, H5O_info1_t, H5Ovisit2, H5_index_t_H5_INDEX_UNKNOWN,
    H5_iter_order_t_H5_ITER_UNKNOWN, H5P_DEFAULT,
};

pub mod h5converter;

pub use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

extern "C" {
    pub static __imp_H5T_C_S1_g : hid_t;
}

const NULL: *mut c_void = 0 as *mut c_void;

const H5F_ACC_RDONLY: u32 = 0;
const H5F_ACC_TRUNC: u32 = 2;


#[allow(temporary_cstring_as_ptr)]
pub fn h5open_file(filename: &str, write: bool) -> Option<hid_t> {
    let file_id;
    unsafe {
        file_id = H5Fopen(
            CString::new(filename).unwrap().as_ptr(),
            if write { H5F_ACC_TRUNC } else { H5F_ACC_RDONLY },
            H5P_DEFAULT.into(),
        );
    }

    if file_id > 0 {
        Some(file_id)
    } else {
        None
    }
}

#[allow(temporary_cstring_as_ptr)]
pub fn h5open_group(file_id: hid_t, name: &str) -> Option<hid_t> {
    let group_id;
    unsafe {
        group_id = H5Gopen2(file_id, CString::new(name).unwrap().as_ptr(), H5P_DEFAULT.into());
    }

    if group_id > 0 {
        Some(group_id)
    } else {
        None
    }
}

unsafe extern "C" fn visit_fun(
    _h5obj: hid_t,
    name: *const c_char,
    _info: *const H5O_info1_t,
    _op_data: *mut c_void,
) -> i32 {
    let c_name = CStr::from_ptr(name);
    println!("{:?}", c_name);
    0
}

pub fn h5tree(h5obj: hid_t) {
    unsafe {
        H5Ovisit2(
            h5obj,
            H5_index_t_H5_INDEX_UNKNOWN,
            H5_iter_order_t_H5_ITER_UNKNOWN,
            Some(visit_fun),
            NULL,
            1,
        );
    }
}
