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
pub mod types;
pub mod utils;

use crate::{
    error::Error as H5Error,
    h5sys::*,
    types::{
        Attr, CFilter, DataSpace, DataSpaceType, DataType, Dataset, File,
        FileOpenAccess, Filter, Group, PList, PListClass,
    },
};

#[derive(Default)]
pub struct Hdf5 {
    filter_counter: std::cell::Cell<i32>,
    version: (u32, u32, u32),
}

impl Hdf5 {
    pub fn init() -> Result<Self, H5Error> {
        unsafe {
            if H5open() < 0 {
                Err(H5Error::library_init_fail())
            } else {
                let mut ret = Self::default();
                if H5get_libversion(
                    &mut ret.version.0,
                    &mut ret.version.1,
                    &mut ret.version.2,
                ) < 0
                {
                    eprintln!(
                        "WARNING: Failed to retrieve hdf5 library version"
                    );
                    ret.version = (0, 0, 0);
                } else {
                    #[cfg(debug_assertions)]
                    println!(
                        "Hdf5 initialized. Version {}.{}.{}",
                        ret.version.0, ret.version.1, ret.version.2
                    );
                }
                ret.filter_counter.replace(512);
                Ok(ret)
            }
        }
    }

    // FILE OPENING
    pub fn create_file(
        &self,
        filename: &str,
        overwrite: bool,
    ) -> Result<File, H5Error> {
        let fid = unsafe {
            H5Fcreate(
                str_to_cchar!(filename),
                if overwrite { H5F_ACC_TRUNC } else { 0 },
                H5P_DEFAULT,
                H5P_DEFAULT,
            )
        };

        if fid <= 0 {
            Err(H5Error::file_create(filename))
        } else {
            Ok(File {
                lib: self,
                filename: filename.to_string(),
                fid,
            })
        }
    }

    pub fn open_file(
        &self,
        filename: &str,
        access: FileOpenAccess,
    ) -> Result<types::File, H5Error> {
        let fid = unsafe {
            H5Fopen(
                str_to_cchar!(filename),
                match access {
                    FileOpenAccess::ReadOnly => H5F_ACC_RDONLY,
                    FileOpenAccess::ReadWrite => H5F_ACC_RDWR,
                },
                H5P_DEFAULT,
            )
        };

        if fid > 0 {
            Ok(File {
                lib: self,
                filename: filename.to_string(),
                fid,
            })
        } else {
            Err(H5Error::file_open(filename))
        }
    }

    // GROUP OPENING

    pub fn open_group(
        &self,
        parent: i64,
        name: &str,
    ) -> Result<Group, H5Error> {
        let g_exists =
            unsafe { H5Lexists(parent, str_to_cchar!(name), H5P_DEFAULT) };

        match g_exists.cmp(&0i32) {
            std::cmp::Ordering::Equal => {
                return Err(H5Error::group_doesnt_exists(name));
            }
            std::cmp::Ordering::Less => {
                return Err(H5Error::group_open(name));
            }
            _ => (),
        }

        let gid = unsafe { H5Gopen2(parent, str_to_cchar!(name), H5P_DEFAULT) };
        if gid <= 0 {
            Err(H5Error::group_open(name))
        } else {
            let path_len = unsafe { H5Iget_name(gid, null_mut(), 0) } as usize;
            let buffer = vec![0usize; path_len + 1];
            unsafe { H5Iget_name(gid, buffer.as_ptr() as _, buffer.len()) };
            let path = unsafe {
                CStr::from_ptr(buffer.as_ptr().cast()).to_str().unwrap()
            };
            Ok(Group {
                lib: self,
                path: path.to_string(),
                gid,
            })
        }
    }

    // DATATYPE OPENING

    pub fn open_type(&self, dtype_id: i64) -> Result<DataType, H5Error> {
        Ok(DataType {
            lib: self,
            tid: dtype_id,
            dtype: DataType::parse_type(dtype_id)?,
        })
    }

    // DATASPACE OPENING
    pub fn open_dataspace(
        &self,
        dataspace_id: i64,
    ) -> Result<DataSpace, H5Error> {
        let n_dims;
        let mut dims = vec![];
        let space_type;
        unsafe {
            n_dims = H5Sget_simple_extent_ndims(dataspace_id);
            if n_dims < 0 {
                return Err(H5Error::dataspace_get_dimensions_fail());
            }
            dims.resize(n_dims as usize, 0usize);
            space_type = if n_dims == 0 {
                DataSpaceType::Scalar
            } else {
                DataSpaceType::Simple
            };
            H5Sget_simple_extent_dims(
                dataspace_id,
                dims.as_ptr().cast_mut().cast(),
                null_mut(),
            );
        }

        Ok(DataSpace {
            lib: self,
            did: dataspace_id,
            space_type,
            dims,
        })
    }

    pub fn create_dataspace(
        &self,
        dataspace_type: DataSpaceType,
        dims: &[u64],
    ) -> Result<DataSpace, H5Error> {
        match dataspace_type {
            DataSpaceType::Simple => {
                let did = unsafe {
                    H5Screate_simple(dims.len() as i32, dims.as_ptr(), null())
                };
                if did > 0 {
                    Ok(DataSpace {
                        lib: self,
                        did,
                        space_type: DataSpaceType::Simple,
                        dims: dims.iter().map(|x| *x as usize).collect(),
                    })
                } else {
                    Err(H5Error::dataspace_simple_new(dims))
                }
            }
            _ => todo!(),
        }
    }

    // DATASET OPENING

    #[allow(unused_unsafe)]
    pub fn open_dataset(
        &self,
        parent: i64,
        name: &str,
    ) -> Result<Dataset, H5Error> {
        let did;
        let path;
        unsafe {
            did = H5Dopen2(parent, str_to_cchar!(name), H5P_DEFAULT);
            if did <= 0 {
                return Err(H5Error::dataset_open_fail(name));
            }

            let path_len = H5Iget_name(did, null_mut(), 0) as usize;
            let buffer = vec![0usize; path_len + 1];
            H5Iget_name(did, buffer.as_ptr() as _, buffer.len());
            path = cchar_to_string!(buffer.as_ptr().cast());
        }

        Ok(Dataset {
            lib: self,
            did,
            path,
            dataspace: Some(self.open_dataspace(did)?),
            datatype: Some(self.open_type(did)?),
        })
    }

    // ATTRIBUTE OPENING
    pub fn open_attribute(
        &self,
        group: i64,
        name: &str,
    ) -> Result<Attr, H5Error> {
        let aid;
        unsafe {
            aid = H5Aopen(group, str_to_cchar!(name), H5P_DEFAULT);
            if aid <= 0 {
                return Err(H5Error::attribute_open(name));
            }
        }

        Ok(Attr {
            lib: self,
            name: name.to_string(),
            aid,
        })
    }

    // FILTER OPENING

    fn get_next_filter_id(&self) -> i32 {
        let ret = self.filter_counter.get();
        self.filter_counter.replace(ret + 1);
        ret
    }

    pub fn create_filter(
        &self,
        function: CFilter,
        desc: Option<&str>,
    ) -> Result<Filter, H5Error> {
        Ok(Filter {
            lib: self,
            fid: self.get_next_filter_id(),
            function,
            desc: desc.unwrap_or("").to_string(),
            n_params: 0,
            flags: 0,
        })
    }

    // PLIST OPENING
    pub fn create_plist(&self, class: PListClass) -> Result<PList, H5Error> {
        let pid = unsafe { H5Pcreate(class.get_id()) };
        if pid <= 0 {
            Err(H5Error::plist_create())
        } else {
            Ok(PList {
                lib: self,
                class,
                pid,
            })
        }
    }
}

impl Drop for Hdf5 {
    fn drop(&mut self) {
        unsafe {
            H5close();
        }
    }
}
