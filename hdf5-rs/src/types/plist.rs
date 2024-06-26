//! This module contains the abstractions to use and create property lists.
//! Those can be passed as arguments to other types functions to enable or
//! disable certain behaviours.
//!
//! # Examples

use crate::{
    error::Error,
    h5sys::{
        H5P_LST_DATASET_ACCESS_ID_g as H5P_DATASET_ACCESS,
        H5P_LST_FILE_ACCESS_ID_g as H5P_FILE_ACCESS,
        H5P_LST_FILE_CREATE_ID_g as H5P_FILE_CREATE, *,
    },
    types::Filter,
    Hdf5,
};

// Enum of the various classes available for property lists. Different
// operations require different classes of property lists
pub enum PListClass {
    AttributeCreate,
    DataSetAccess,
    DataSetCreate,
    DataSetTrasnfer,
    DataTypeAccess,
    DataTypeCreate,
    FileAccess,
    FileCreate,
    GroupAccess,
    GroupCreate,
    LinkAccess,
    LinkCreate,
    StringCreate,
    None,
}

impl PListClass {
    pub fn get_id(&self) -> i64 {
        unsafe {
            H5open();
        }

        match self {
            PListClass::DataSetAccess => unsafe { H5P_DATASET_ACCESS },
            PListClass::FileAccess => unsafe { H5P_FILE_ACCESS },
            PListClass::FileCreate => unsafe { H5P_FILE_CREATE },
            _ => todo!(),
        }
    }
}

pub struct PList<'lib> {
    pub lib: &'lib Hdf5,
    pub class: PListClass,
    pub pid: i64,
}

impl<'lib> PList<'lib> {
    pub fn copy(_pid: i64) -> Result<Self, String> {
        todo!()
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
