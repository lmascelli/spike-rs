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
//! use hdf5_rs::types::{File, FileOpenAccess};
//!
//! fn open_file(filename: &str) -> Result<(), String> {
//!     let file = File::open(filename, FileOpenAccess::ReadOnly)?;
//!     let group = file.open_group("group_name")?;
//!     let dataset = group.get_dataset("dataset_name")?;
//!
//!     // print information about the objects
//!     println!("{file}");
//!     println!("{group}");
//!     println!("{dataset}");
//!
//!     Ok(())
//! }
//! ```

pub mod h5sys;
pub mod types;
pub mod utils;
