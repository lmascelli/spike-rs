//! This module contains the abstractions to use and create property lists.
//! Those can be passed as arguments to other types functions to enable or
//! disable certain behaviours.
//!
//! # Examples

use crate::{cchar_to_string, error::Error, h5sys::*, types::Filter, Hdf5};

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
            PListClass::DataSetCreate => unsafe { H5P_CLS_DATASET_CREATE_ID_g },
            PListClass::DataSetAccess => unsafe { H5P_CLS_DATASET_ACCESS_ID_g },
            PListClass::FileAccess => unsafe { H5P_CLS_FILE_ACCESS_ID_g },
            PListClass::FileCreate => unsafe { H5P_CLS_FILE_CREATE_ID_g },
            _ => todo!(),
        }
    }

    pub fn from_id(id: i64) -> Self {
        let class_id = unsafe { H5Pget_class(id) };
        let class_name = cchar_to_string!(H5Pget_class_name(class_id));
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

pub struct PList<'lib> {
    pub lib: &'lib Hdf5,
    pub pid: i64,
    pub class: PListClass,
}

impl<'lib> PList<'lib> {
    pub fn check_class(&self, class: PListClass) -> Result<(), Error> {
        if unsafe { H5Pequal(self.class.get_id(), class.get_id()) } > 0 {
            Ok(())
        } else {
            Err(Error::plist_classes_do_not_match(
                &format!("{:?}", self.class),
                &format!("{:?}", class),
            ))
        }
    }

    pub fn get_pid(&self) -> i64 {
        self.pid
    }

    pub fn set_filter(
        &mut self,
        filter: &Filter,
        params: &[u32],
    ) -> Result<(), Error> {
        if filter.n_params != params.len() {
            Err(Error::plist_set_filter_wrong_number_of_parameters())
        } else {
            unsafe {
                H5Pset_filter(
                    self.get_pid(),
                    filter.get_fid(),
                    filter.flags,
                    filter.n_params,
                    params.as_ptr(),
                );
            }

            Ok(())
        }
    }

    // DATASET CREATE PLIST

    pub fn get_chunk(&self, ndims: usize) -> Result<Vec<usize>, Error> {
        match self.class {
            PListClass::DataSetCreate => {
                let dims = vec![0u64; ndims];
                unsafe {
                    if H5Pget_chunk(
                        self.pid,
                        ndims as _,
                        dims.as_ptr().cast_mut(),
                    ) < 0
                    {
                        Err(Error::plist_get_chunk_fail())
                    } else {
                        Ok(dims.iter().map(|x| *x as usize).collect())
                    }
                }
            }
            _ => Err(Error::plist_not_dataset_access()),
        }
    }
}

impl<'lib> Drop for PList<'lib> {
    fn drop(&mut self) {
        if self.pid > 0 {
            unsafe {
                H5Pclose(self.pid);
            }
        }
    }
}
