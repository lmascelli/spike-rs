use crate::error::Error;
use crate::h5sys::*;
use crate::types::{
    group::{Group, GroupOpener},
    plist::PList,
};
use crate::utils::{get_group_names, str_to_cchar};
use crate::H5LibRuntime;

pub enum FileOpenAccess {
    ReadOnly,
    ReadWrite,
}

#[allow(unused)]
pub struct File<'runtime> {
    runtime: &'runtime H5LibRuntime,
    filename: String,
    fid: i64,
}

impl<'runtime> File<'runtime> {
    pub fn create(
        runtime: &'runtime H5LibRuntime,
        filename: &str,
        overwrite: bool,
    ) -> Result<Self, Error> {
        let fid = unsafe {
            H5Fcreate(
                str_to_cchar!(filename),
                if overwrite { H5F_ACC_TRUNC } else { 0 },
                H5P_DEFAULT,
                H5P_DEFAULT,
            )
        };

        if fid <= 0 {
            Err(Error::file_create(filename))
        } else {
            Ok(Self {
                runtime,
                filename: filename.to_string(),
                fid,
            })
        }
    }

    pub fn open(
        runtime: &'runtime H5LibRuntime,
        filename: &str,
        access: FileOpenAccess,
    ) -> Result<Self, Error> {
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
                runtime,
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

    pub fn is_accessible(filename: &str, plist: &PList) -> bool {
        let res = unsafe {
            H5Fis_accessible(str_to_cchar!(filename), plist.get_pid())
        };
        if res <= 0 {
            false
        } else {
            true
        }
    }
}

impl<'runtime> Drop for File<'runtime> {
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

impl<'runtime> std::fmt::Display for File<'runtime> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5File")?;
        writeln!(f, "  filename: {}", self.filename)?;
        writeln!(f, "  fid {}", self.fid)?;
        Ok(())
    }
}

impl<'runtime> GroupOpener for File<'runtime> {
    fn open_group(&self, name: &str) -> Result<Group, Error> {
        Group::open(self.get_fid(), name)
    }

    fn list_groups(&self) -> Vec<String> {
        get_group_names(self.fid)
    }
}
