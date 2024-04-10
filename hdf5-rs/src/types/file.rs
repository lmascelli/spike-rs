use crate::h5sys::*;
use crate::utils::{str_to_cchar, get_group_names};
use crate::types::group::{Group, GroupOpener};

pub struct File {
    filename: String,
    fid: i64,
}

impl File {
    pub fn open(filename: &str) -> Result<Self, String> {
        let fid = unsafe { 
            H5Fopen(
                str_to_cchar!(filename),
                H5F_ACC_RDONLY,
                H5P_DEFAULT,
            )
        };
        Ok(File {
            filename: filename.to_string(),
            fid,
        })
    }

    pub fn get_fid(&self) -> i64 {
        self.fid
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if self.fid > 0 {
            #[cfg(debug_assertions)] {
                println!("Closing file: {}", self.filename);
            }
            unsafe { H5Fclose(self.fid); }
        }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5File")?;
        writeln!(f, "  filename: {}", self.filename)?;
        writeln!(f, "  fid {}", self.fid)?;
        Ok(())
    }
}

impl GroupOpener for File {
    fn open_group(&self, name: &str) -> Result<Group, String> {
        if let Ok(group) = Group::open(self.get_fid(), name) {
            Ok(group)
        } else {
            Err(format!("Failed opening group {}", name))
        }
    }

    fn list_groups(&self) -> Vec<String> {
        get_group_names(self.fid)
    }
}
