//! This module contains the abstractions to use and create property lists.
//! Those can be passed as arguments to other types functions to enable or
//! disable certain behaviours.
//!
//! # Examples

use crate::{
    cchar_to_string, error::H5Error, h5sys::*, identifiers::is_valid_id,
    types::Filter,
};

// Enum of the various classes available for property lists. Different
// operations require different classes of property lists
#[derive(Debug)]
pub enum PListClass {
    AttributeCreate,
    DataSetAccess,
    DataSetCreate,
    DataSetTransfer,
    DataTypeAccess,
    DataTypeCreate,
    FileAccess,
    FileCreate,
    FileMount,
    GroupAccess,
    GroupCreate,
    LinkAccess,
    LinkCreate,
    ObjectCopy,
    ObjectCreate,
    StringCreate,
    None,
}

impl PListClass {
    pub fn get_id(&self) -> i64 {
        match self {
            PListClass::DataSetCreate => unsafe {
                plist::H5P_CLS_DATASET_CREATE_ID_g
            },
            PListClass::DataSetAccess => unsafe {
                plist::H5P_CLS_DATASET_ACCESS_ID_g
            },
            PListClass::DataSetTransfer => unsafe {
                plist::H5P_CLS_DATASET_XFER_ID_g
            },
            PListClass::FileAccess => unsafe {
                plist::H5P_CLS_FILE_ACCESS_ID_g
            },
            PListClass::FileCreate => unsafe {
                plist::H5P_CLS_FILE_CREATE_ID_g
            },
            _ => todo!(),
        }
    }

    pub fn from_id(id: i64) -> Self {
        let class_id = unsafe { plist::H5Pget_class(id) };
        let class_name = cchar_to_string!(plist::H5Pget_class_name(class_id));
        match class_name.as_str() {
            "attribute create" => Self::AttributeCreate,
            "dataset access" => Self::DataSetAccess,
            "dataset create" => Self::DataSetCreate,
            "data transfer" => Self::DataSetTransfer,
            "datatype access" => Self::DataTypeAccess,
            "datatype create" => Self::DataTypeCreate,
            "file access" => Self::FileAccess,
            "file create" => Self::FileCreate,
            "file mount" => Self::FileMount,
            "group access" => Self::GroupAccess,
            "group create" => Self::GroupCreate,
            "link access" => Self::LinkAccess,
            "link create" => Self::LinkCreate,
            "object copy" => Self::ObjectCopy,
            "object create" => Self::ObjectCreate,
            "string create" => Self::StringCreate,
            _ => Self::None,
        }
    }
}

pub struct PList {
    pub pid: types::Hid,
    pub class: PListClass,
}

impl PList {
    pub fn create(class: PListClass) -> Result<Self, H5Error> {
        let pid = unsafe { plist::H5Pcreate(class.get_id()) };
        if pid <= 0 {
            Err(H5Error::plist_create())
        } else {
            Ok(Self { class, pid })
        }
    }

    pub fn check_class(&self, class: PListClass) -> Result<(), H5Error> {
        if unsafe { plist::H5Pequal(self.class.get_id(), class.get_id()) } > 0 {
            Ok(())
        } else {
            Err(H5Error::plist_classes_do_not_match(
                &format!("{:?}", self.class),
                &format!("{:?}", class),
            ))
        }
    }

    pub fn get_pid(&self) -> types::Hid {
        self.pid
    }

    pub fn set_filter(
        &mut self,
        filter: &Filter,
        params: &[u32],
    ) -> Result<(), H5Error> {
        if filter.n_params != params.len() {
            Err(H5Error::plist_set_filter_wrong_number_of_parameters())
        } else {
            unsafe {
                plist::H5Pset_filter(
                    self.pid,
                    filter.fid,
                    filter.flags,
                    filter.n_params,
                    params.as_ptr(),
                );
            }

            Ok(())
        }
    }

    // DATASET CREATE PLIST

    pub fn get_chunk(&self, ndims: usize) -> Result<Vec<usize>, H5Error> {
        match self.class {
            PListClass::DataSetCreate => {
                let dims = vec![0u64; ndims];
                unsafe {
                    if plist::H5Pget_chunk(
                        self.pid,
                        ndims as _,
                        dims.as_ptr().cast_mut(),
                    ) < 0
                    {
                        Err(H5Error::plist_get_chunk_fail())
                    } else {
                        Ok(dims.iter().map(|x| *x as usize).collect())
                    }
                }
            }
            _ => Err(H5Error::plist_not_dataset_access()),
        }
    }
}

impl Drop for PList {
    fn drop(&mut self) {
        if self.pid > 0 {
            unsafe {
                if is_valid_id(self.pid) {
                    plist::H5Pclose(self.pid);
                }
            }
        }
    }
}
