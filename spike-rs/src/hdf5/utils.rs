////////////////////////////////////////////////////////////////////////////////
///
///                                   Utils
///
////////////////////////////////////////////////////////////////////////////////

use crate::hdf5::sys::*;

////////////////////////////////////////////////////////////////////////////////
//                                 CONVERSIONS

pub fn str_to_cchar(s: &str) -> *const c_char {
    assert!(s.ends_with("\0"), "str_to_cchar: the string to convert should end with \\0");
    CStr::from_bytes_with_nul(s.as_bytes()).unwrap().as_ptr()
}

pub fn cchar_to_string(s: *const i8) -> String {
    unsafe { CStr::from_ptr(s).to_str().unwrap().to_string() }
}

////////////////////////////////////////////////////////////////////////////////
//                                NAMES IN GROUPS 

extern "C" fn _get_group_name(_group: i64,
                                name:   *const i8,
                                _info:  *const H5L_info2_t,
                                data:  *mut c_void,
                                ) -> i32 {
    let name_v;
    let data_v;
    unsafe {
	data_v = &mut*(data as *mut Vec<String>);
	name_v = CStr::from_ptr(name).to_str().unwrap().to_string();
    }
    data_v.push(name_v);
    0
}

#[allow(unused)]
pub fn get_group_names(group: i64) -> Vec<String> {
    let ret = vec![];
    unsafe {
        H5Literate2(group,
                    H5_index_t_H5_INDEX_NAME,
                    H5_iter_order_t_H5_ITER_INC,
                    null_mut(),
                    Some(_get_group_name),
                    &ret as *const Vec<String> as *const c_void as *mut c_void);
    }
    ret
}

////////////////////////////////////////////////////////////////////////////////
//                            ATTRIBUTES IN GROUPS 

extern "C" fn _get_attribute_name(_group: i64,
                                    name:   *const i8,
                                    _info:  *const H5A_info_t,
                                    data:  *mut c_void,
                                    ) -> i32 {
    let name_v;
    let data_v;
    unsafe {
	data_v = &mut*(data as *mut Vec<String>);
	name_v = CStr::from_ptr(name).to_str().unwrap().to_string();
    }
    data_v.push(name_v);
    0
}

#[allow(unused)]
pub fn get_attribute_names(group: i64) -> Vec<String> {
    let ret = vec![];
    unsafe {
        let x = H5Aiterate2(group,
                    H5_index_t_H5_INDEX_NAME,
                    H5_iter_order_t_H5_ITER_INC,
                    null_mut(),
                    Some(_get_attribute_name),
                    &ret as *const Vec<String> as *const c_void as *mut c_void);
    }
    ret
}

pub fn get_attr_array_dim(group_id: i64, attr_name: &str) -> Option<Vec<usize>> {
    let ret = vec![];
    let n_dims;
    unsafe {
        let attr_id = H5Aopen(group_id, str_to_cchar(attr_name), H5P_DEFAULT);
        if attr_id <= 0 {
            return None;
        }
        let attr_space = H5Aget_space(attr_id);
        n_dims = H5Sget_simple_extent_ndims(attr_space);
        println!("{n_dims}");
        H5Sclose(attr_space);
        H5Aclose(attr_id);
    }
    Some(ret)

}

pub fn get_attr_str(group_id: i64, attr_name: &str, ssize: usize) -> Option<String> {
    let ret;
    unsafe {
        let attr_id = H5Aopen(group_id, str_to_cchar(attr_name), H5P_DEFAULT);
        if attr_id <= 0 {
            return None;
        }
        let attr_type = H5Aget_type(attr_id);
        if attr_type <= 0 {
            H5Aclose(attr_id);
            return None;
        }

        let data = vec![0usize; ssize];
        H5Aread(attr_id, attr_type, data.as_ptr().cast_mut() as *mut c_void);
        H5Tclose(attr_type);
        H5Aclose(attr_id);

        if let Ok(s) = CStr::from_ptr(data.as_ptr().cast()).to_str() {
            ret = s.to_string();
        } else {
            return None;
        }
    } 
    Some(ret)
}

pub fn get_attr_ulong(group_id: i64, attr_name: &str) -> Option<u64> {
    let ret;
    unsafe {
        let attr_id = H5Aopen(group_id, str_to_cchar(attr_name), H5P_DEFAULT);
        if attr_id <= 0 {
            return None;
        }
        let attr_type = H5Aget_type(attr_id);
        if attr_type <= 0 {
            H5Aclose(attr_id);
            return None;
        }

        ret = 0u64;
        H5Aread(attr_id, attr_type, &ret as *const u64 as *mut c_void);
        H5Tclose(attr_type);
        H5Aclose(attr_id);
    } 
    Some(ret)
}

pub fn get_attr_ilong(group_id: i64, attr_name: &str) -> Option<i64> {
    let ret;
    unsafe {
        let attr_id = H5Aopen(group_id, str_to_cchar(attr_name), H5P_DEFAULT);
        if attr_id <= 0 {
            return None;
        }
        let attr_type = H5Aget_type(attr_id);
        if attr_type <= 0 {
            H5Aclose(attr_id);
            return None;
        }

        ret = 0i64;
        H5Aread(attr_id, attr_type, &ret as *const i64 as *mut c_void);
        H5Tclose(attr_type);
        H5Aclose(attr_id);
    } 
    Some(ret)
}
