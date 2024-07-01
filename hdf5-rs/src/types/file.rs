use crate::{
    error::H5Error,
    h5sys::*,
    types::{
        group::{Group, GroupOpener},
        plist::PList,
    },
    utils::{get_group_names, str_to_cchar},
};

pub enum FileOpenAccess {
    ReadOnly,
    ReadWrite,
}

#[allow(unused)]
pub struct File {
    pub filename: String,
    pub fid: i64,
}

impl File {
    pub fn create(filename: &str, overwrite: bool) -> Result<Self, H5Error> {
        let fid = unsafe {
            file::H5Fcreate(
                str_to_cchar!(filename),
                if overwrite { file::H5F_ACC_TRUNC } else { 0 },
                plist::H5P_DEFAULT,
                plist::H5P_DEFAULT,
            )
        };

        if fid <= 0 {
            Err(H5Error::file_create(filename))
        } else {
            Ok(Self { filename: filename.to_string(), fid })
        }
    }

    pub fn open(
        filename: &str,
        access: FileOpenAccess,
    ) -> Result<Self, H5Error> {
        let fid = unsafe {
            file::H5Fopen(
                str_to_cchar!(filename),
                match access {
                    FileOpenAccess::ReadOnly => file::H5F_ACC_RDONLY,
                    FileOpenAccess::ReadWrite => file::H5F_ACC_RDWR,
                },
                plist::H5P_DEFAULT,
            )
        };

        if fid > 0 {
            Ok(File { filename: filename.to_string(), fid })
        } else {
            Err(H5Error::file_open(filename))
        }
    }

    pub fn get_fid(&self) -> i64 {
        self.fid
    }

    pub fn is_accessible(filename: &str, plist: &PList) -> bool {
        let res = unsafe {
            file::H5Fis_accessible(str_to_cchar!(filename), plist.get_pid())
        };
        if res <= 0 {
            false
        } else {
            true
        }
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
                file::H5Fclose(self.fid);
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
    fn open_group(&self, name: &str) -> Result<Group, H5Error> {
        Group::open(self.get_fid(), name)
    }

    fn list_groups(&self) -> Vec<String> {
        get_group_names(self.fid)
    }
}
