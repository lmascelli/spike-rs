use crate::{
    error::Error,
    h5sys::*,
    types::{
        group::{Group, GroupOpener},
        plist::PList,
    },
    utils::{get_group_names, str_to_cchar},
    Hdf5,
};

pub enum FileOpenAccess {
    ReadOnly,
    ReadWrite,
}

#[allow(unused)]
pub struct File<'lib> {
    pub lib: &'lib Hdf5,
    pub filename: String,
    pub fid: i64,
}

impl<'lib> File<'lib> {
    pub fn create(
        lib: &'lib Hdf5,
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
                lib,
                filename: filename.to_string(),
                fid,
            })
        }
    }

    pub fn open(lib: &'lib Hdf5, filename: &str, access: FileOpenAccess) -> Result<Self, Error> {
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
                lib,
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

impl<'lib> Drop for File<'lib> {
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

impl<'lib> std::fmt::Display for File<'lib> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5File")?;
        writeln!(f, "  filename: {}", self.filename)?;
        writeln!(f, "  fid {}", self.fid)?;
        Ok(())
    }
}

impl<'lib> GroupOpener for File<'lib> {
    fn open_group(&self, name: &str) -> Result<Group, Error> {
        self.lib.open_group(self.get_fid(), name)
    }

    fn list_groups(&self) -> Vec<String> {
        get_group_names(self.fid)
    }
}
