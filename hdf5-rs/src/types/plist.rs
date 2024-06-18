//! This module contains the abstractions to use and create property lists.
//! Those can be passed as arguments to other types functions to enable or
//! disable certain behaviours.
//!
//! # Examples

use crate::h5sys::*;
use crate::error::Error;
use crate::h5sys::{
    H5P_LST_FILE_ACCESS_ID_g as H5P_FILE_ACCESS, H5P_LST_FILE_CREATE_ID_g as H5P_FILE_CREATE,
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
}

pub struct PList {
    class: i64,
    pid: i64,
}

impl PList {
    pub fn create(class: i64) -> Result<Self, Error> {
        let pid = unsafe { H5Pcreate(class) };
        if pid <= 0 {
            Err(Error::plist_create())
        } else {
            Ok(Self { class, pid })
        }
    }

    pub fn copy(pid: i64) -> Result<Self, String> {
        todo!()
    }

    pub fn get_pid(&self) -> i64 {
        self.pid
    }
}

impl Drop for PList {
    fn drop(&mut self) {
        if self.pid > 0 {
            unsafe {
                H5Pclose(self.pid);
            }
        }
    }
}
