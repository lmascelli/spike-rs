use crate::error::{Error, ErrorType};
use crate::h5sys::*;
use crate::types::group::{Group, GroupOpener};
use crate::utils::{get_group_names, str_to_cchar};

pub enum FileOpenAccess {
    ReadOnly,
    ReadWrite,
}

pub struct File {
    filename: String,
    fid: i64,
}

impl File {
    pub fn open(filename: &str, access: FileOpenAccess) -> Result<Self, Error> {
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
                filename: filename.to_string(),
                fid,
            })
        } else {
            Err(Error::file_open(filename))
        }
    }

    pub fn get_fid(&self) -> i64 {
        self.fid
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if self.fid > 0 {
            #[cfg(debug_assertions)]
            {
                println!("Closing file: {}", self.filename);
            }
            unsafe {
                H5Fclose(self.fid);
            }
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
    fn open_group(&self, name: &str) -> Result<Group, Error> {
        Group::open(self.get_fid(), name) 
    }

    fn list_groups(&self) -> Vec<String> {
        get_group_names(self.fid)
    }
}
