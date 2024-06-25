//! A wrapper around the C hdf5 library
//!
//! Goal of this crate is to provide an easy to use interface around the
//! [hdf5](https://www.hdfgroup.org/solutions/hdf5/) C library with an abstraction of the main
//! structures (File, Group, Attribute, Dataset, etc...) that have associated methods and
//! automatically release of the resources one out of scope.
//!
//! Most of the errors of this crate are (temporarely) handled as String which contain
//! the information retrieved about it. Maybe in the future a better implementation of
//! errors as dedicated enum will be done.
//!
//! # Examples
//!
//! here it is an example of opening a file with `ReadOnly` permissions and
//! accessing a dataset in a group in it:
//!
//! ```
//! use hdf5_rs::{
//!     types::{File, FileOpenAccess, GroupOpener, DatasetOwner},
//!     error::{Error},
//! };
//!
//! fn open_file(filename: &str) -> Result<(), Error> {
//!     let file = File::open(filename, FileOpenAccess::ReadOnly)?;
//!     let group = file.open_group("group_name")?;
//!     let dataset = group.get_dataset("dataset_name").unwrap();
//!
//!     // print information about the objects
//!     println!("{file}");
//!     println!("{group}");
//!     println!("{dataset}");
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod h5sys;
pub mod types;
pub mod utils;

use error::Error;
use h5sys::{H5close, H5get_libversion, H5open};

pub struct H5LibRuntime {}

impl H5LibRuntime {
    pub fn new() -> Result<Self, Error> {
        let open_success;
        let version_success;
        let mut version_maior: u32 = 0;
        let mut version_minor: u32 = 0;
        let mut version_number: u32 = 0;
        unsafe {
            open_success = H5open();
        }
        if open_success < 0 {
            Err(Error::library_init_fail())
        } else {
            unsafe {
                version_success = H5get_libversion(
                    &mut version_maior as *mut u32,
                    &mut version_minor as *mut u32,
                    &mut version_number as *mut u32,
                );

                #[cfg(debug_assertions)]
                if version_success < 0 {
                    eprintln!("WARNING: FAILED TO RETRIEVE HDF5 VERSION");
                } else {
                    println!("HDF5 version: {}.{}.{}", version_maior, version_minor, version_number);
                }
            }
            Ok(Self {})
        }
    }
}

impl Drop for H5LibRuntime {
    fn drop(&mut self) {
        unsafe {
            H5close();
        }
    }
}
