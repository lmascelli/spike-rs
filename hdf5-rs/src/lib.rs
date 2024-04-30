//! A wrapper around the C hdf5 library
//!
//! Goal of this crate is to provide an easy to use interface around the
//! [hdf5](https://www.hdfgroup.org/solutions/hdf5/) C library with an abstraction of the main
//! structures (File, Group, Attribute, Dataset, etc...) that have associated methods and
//! automatically release of the resources one out of scope.

pub mod h5sys;
pub mod types;
pub mod utils;
