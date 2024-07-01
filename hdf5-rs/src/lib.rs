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
//!     error::{Error as H5Error},
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
pub mod identifiers;
pub mod prelude;
pub mod types;
pub mod utils;

use crate::{error::H5Error, h5sys::*};

/// Initialize the HDF5 library
pub fn init() -> Result<(u32, u32, u32), H5Error> {
    unsafe {
        let mut ret = (0, 0, 0);
        if lib::H5open() < 0 {
            Err(H5Error::library_init_fail())
        } else {
            if lib::H5get_libversion(&mut ret.0, &mut ret.1, &mut ret.2) < 0 {
                eprintln!("WARNING: Failed to retrieve hdf5 library version");
                ret = (0, 0, 0);
            } else {
                #[cfg(debug_assertions)]
                println!(
                    "Hdf5 initialized. Version {}.{}.{}",
                    ret.0, ret.1, ret.2
                );
            }
            Ok(ret)
        }
    }
}

/// Deinitialize the HDF5 library and free the memory
pub fn close() {
    unsafe {
        lib::H5close();
    }
}
