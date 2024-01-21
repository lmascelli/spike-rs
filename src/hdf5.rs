pub use crate::sys::{
    hid_t, H5Dclose, H5Dcreate2, H5Fclose, H5Fcreate, H5Fopen, H5Gclose, H5Gcreate2, H5Gopen2,
    H5O_info1_t, H5Ovisit2, H5Sclose, H5Tclose, H5_index_t_H5_INDEX_UNKNOWN,
    H5_iter_order_t_H5_ITER_UNKNOWN, H5P_DEFAULT,
};
use crate::sys::{H5Screate_simple, H5T_NATIVE_FLOAT_g, H5Tcopy, H5Tset_size};
pub use cstr::cstr;
pub use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};


pub mod h5converter;

const NULL: *mut c_void = 0 as *mut c_void;

const H5F_ACC_RDONLY: u32 = 0;
const H5F_ACC_TRUNC: u32 = 2;

///////////////////////////////////////////////////////////////////////////////
//
//                            H5 EXPORT STRUCTS
//
///////////////////////////////////////////////////////////////////////////////

pub struct H5Field {
    dataset: hid_t,
    dataspace: hid_t,
    datatype: hid_t,
}

impl H5Field {
    pub fn new(dataset: hid_t, dataspace: hid_t, datatype: hid_t) -> H5Field {
        H5Field {
            dataset,
            dataspace,
            datatype,
        }
    }
}

impl Drop for H5Field {
    fn drop(&mut self) {
        unsafe {
            H5Tclose(self.datatype);
            H5Sclose(self.dataspace);
            H5Dclose(self.dataset);
        }
    }
}

pub trait IntoH5Field {
    fn into_h5field(&self, group: hid_t, name: &str) -> Option<H5Field>;
}

impl IntoH5Field for &[f32] {
    fn into_h5field(&self, group: hid_t, name: &str) -> Option<H5Field> {
        let name = CString::new(name).ok()?;
        let datatype;
        let dataspace;
        let dataset;
        unsafe {
            datatype = H5Tcopy(H5T_NATIVE_FLOAT_g);
            H5Tset_size(datatype, 4);

            let dsize = [self.len()];
            dataspace = H5Screate_simple(1, dsize.as_ptr().cast(), 0 as _);

            dataset = H5Dcreate2(
                group,
                name.as_bytes().as_ptr().cast(),
                datatype,
                dataspace,
                H5P_DEFAULT.into(),
                H5P_DEFAULT.into(),
                H5P_DEFAULT.into(),
            );
        }
        Some(H5Field::new(dataset, dataspace, datatype))
    }
}

pub struct H5Struct {
    fields: Vec<H5Field>,
    structs: Vec<H5Struct>,
    id: hid_t,
}

impl H5Struct {
    fn new(group: hid_t, name: &str) -> Option<Self> {
        let name = CString::new(name).ok()?;
        unsafe {
            Some(H5Struct {
                structs: Vec::new(),
                id: H5Gcreate2(
                    group,
                    name.as_bytes().as_ptr().cast(),
                    H5P_DEFAULT.into(),
                    H5P_DEFAULT.into(),
                    H5P_DEFAULT.into(),
                ),
            })
        }
    }

    fn add_struct(&mut self, name: &str) -> Option<&H5Struct> {
        self.structs.push(H5Struct::new(self.id, name)?);
        Some(&self.structs[self.structs.len() - 1])
    }
}

impl Drop for H5Struct {
    fn drop(&mut self) {
        unsafe {
            H5Gclose(self.id);
        }
    }
}

pub struct H5File {
    fields: Vec<H5Struct>,
    id: hid_t,
}

impl H5File {
    pub fn create(filepath: &str) -> Option<Self> {
        let id;
        let filepath = CString::new(filepath).unwrap();
        unsafe {
            id = H5Fcreate(
                filepath.as_bytes().as_ptr().cast(),
                H5F_ACC_TRUNC,
                H5P_DEFAULT as i64,
                H5P_DEFAULT as i64,
            );
        }
        if id <= 0 {
            return None;
        }

        Some(Self {
            fields: Vec::new(),
            id,
        })
    }

    pub fn add_struct(&mut self, name: &str) -> Option<&H5Struct> {
        self.fields.push(H5Struct::new(self.id, name)?);
        Some(&self.fields[self.fields.len() - 1])
    }
}

impl Drop for H5File {
    fn drop(&mut self) {
        unsafe {
            H5Fclose(self.id);
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//
//                            H5 EXPORT UTILITY
//
///////////////////////////////////////////////////////////////////////////////

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
        group_id = H5Gopen2(
            file_id,
            CString::new(name).unwrap().as_ptr(),
            H5P_DEFAULT.into(),
        );
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
