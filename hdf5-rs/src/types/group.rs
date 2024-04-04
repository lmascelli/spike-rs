use crate::h5sys::*;
use crate::utils::{str_to_cchar, get_group_names};
use crate::types::{
    Attr,
    AttrOpener,
    Dataset,
    DatasetOwner,
};

pub struct Group {
    path: String,
    gid: i64,
}

impl Group {
    pub fn open(parent: i64, name: &str) -> Result<Self, String> {
        let gid = unsafe { H5Gopen2(
            parent,
            str_to_cchar!(name),
            H5P_DEFAULT,
        ) };
        if gid <= 0 {
            Err(format!("Group::open: failed to opening group: {}", name))
        } else {
            let path_len = unsafe { H5Iget_name(gid, null_mut(), 0) } as usize;
            let buffer = vec![0usize; path_len + 1];
            unsafe { H5Iget_name(gid, buffer.as_ptr() as _, buffer.len()) };
            let path = unsafe {CStr::from_ptr(buffer.as_ptr().cast()).to_str().unwrap() };
            Ok( Self {
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
    fn open_group(&self, name: &str) -> Result<Group, String>;
    fn list_groups(&self) -> Vec<String>;
}

impl Drop for Group {
    fn drop(&mut self) {
        if self.gid > 0 {
            println!("Closing group: {}", self.path);
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
    fn open_attr(&self, name: &str) -> Result<Attr, String> {
        Attr::open(self.get_gid(), name)
    }
}

impl DatasetOwner for Group {
    fn get_dataset(&self, name: &str) -> Result<Dataset, String> {
        Dataset::open(self.gid, name)
    }
}

impl GroupOpener for Group {
    fn open_group(&self, name: &str) -> Result<Group, String> {
        if let Ok(group) = Group::open(self.get_gid(), name) {
            Ok(group)
        } else {
            Err(format!("Failed opening group {}", name))
        }
    }

    fn list_groups(&self) -> Vec<String> {
        get_group_names(self.gid)
    }
}
