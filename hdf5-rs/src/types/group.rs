use crate::h5sys::*;
use crate::types::{Attr, AttrOpener, Dataset, DatasetOwner};
use crate::utils::{get_group_names, str_to_cchar};
use crate::error::{Error, ErrorType};

pub struct Group {
    path: String,
    gid: i64,
}

impl Group {
    pub fn open(parent: i64, name: &str) -> Result<Self, Error> {
        let g_exists = unsafe { H5Lexists(parent, str_to_cchar!(name), H5P_DEFAULT) };

        match g_exists.cmp(&0i32) {
            std::cmp::Ordering::Equal => {
                return Err(Error::group_doesnt_exists(name));
            },
            std::cmp::Ordering::Less => {
                return Err(Error::group_open(name));
            },
            _ => (),
        }

        let gid = unsafe { H5Gopen2(parent, str_to_cchar!(name), H5P_DEFAULT) };
        if gid <= 0 {
            Err(Error::group_open(name))
        } else {
            let path_len = unsafe { H5Iget_name(gid, null_mut(), 0) } as usize;
            let buffer = vec![0usize; path_len + 1];
            unsafe { H5Iget_name(gid, buffer.as_ptr() as _, buffer.len()) };
            let path = unsafe { CStr::from_ptr(buffer.as_ptr().cast()).to_str().unwrap() };
            Ok(Self {
                path: path.to_string(),
                gid,
            })
        }
    }

    pub fn get_gid(&self) -> i64 {
        self.gid
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}

pub trait GroupOpener {
    fn open_group(&self, name: &str) -> Result<Group, Error>;
    fn list_groups(&self) -> Vec<String>;
}

impl Drop for Group {
    fn drop(&mut self) {
        if self.gid > 0 {
            #[cfg(debug_assertions)]
            {
                println!("Closing group: {}", self.path);
            }
            unsafe { H5Gclose(self.gid) };
        }
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5Group")?;
        writeln!(f, "  path: {}", self.path)?;
        writeln!(f, "  gid {}", self.gid)?;
        Ok(())
    }
}

impl AttrOpener for Group {
    fn open_attr(&self, name: &str) -> Result<Attr, Error> {
        Attr::open(self.get_gid(), name)
    }
}

impl DatasetOwner for Group {
    fn get_dataset(&self, name: &str) -> Result<Dataset, Error> {
        Dataset::open(self.gid, name)
    }
    fn list_datasets(&self) -> Vec<String> {
        get_group_names(self.gid)
    }
}

impl GroupOpener for Group {
    fn open_group(&self, name: &str) -> Result<Group, Error> {
        Group::open(self.get_gid(), name)
    }

    fn list_groups(&self) -> Vec<String> {
        get_group_names(self.gid)
    }
}
