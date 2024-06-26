use crate::types::{Attr, AttrOpener, Dataset, DatasetOwner};
use crate::{error::Error, utils::get_group_names};
use crate::{h5sys::*, Hdf5};

pub struct Group<'lib> {
    pub lib: &'lib Hdf5,
    pub path: String,
    pub gid: i64,
}

impl<'lib> Group<'lib> {
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

impl<'lib> Drop for Group<'lib> {
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

impl<'lib> std::fmt::Display for Group<'lib> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5Group")?;
        writeln!(f, "  path: {}", self.path)?;
        writeln!(f, "  gid {}", self.gid)?;
        Ok(())
    }
}

impl<'lib> AttrOpener for Group<'lib> {
    fn open_attr(&self, name: &str) -> Result<Attr, Error> {
        self.lib.open_attribute(self.get_gid(), name)
    }
}

impl<'lib> DatasetOwner for Group<'lib> {
    fn get_dataset(&self, name: &str) -> Result<Dataset, Error> {
        self.lib.open_dataset(self.gid, name)
    }
    fn list_datasets(&self) -> Vec<String> {
        get_group_names(self.gid)
    }
}

impl<'lib> GroupOpener for Group<'lib> {
    fn open_group(&self, name: &str) -> Result<Group, Error> {
        self.lib.open_group(self.get_gid(), name)
    }

    fn list_groups(&self) -> Vec<String> {
        get_group_names(self.gid)
    }
}
