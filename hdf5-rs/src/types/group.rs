use crate::types::{
    exists, Attr, AttrOpener, CreateDataSetOptions, DataSet, DatasetOwner,
    PList,
};
use crate::{
    error::H5Error, utils::get_datasets_names, utils::get_group_names,
};
use crate::{h5sys::*, str_to_cchar};

#[derive(Debug)]
pub struct Group {
    pub path: String,
    pub gid: i64,
}

impl Group {
    pub fn open(parent: i64, name: &str) -> Result<Self, H5Error> {
        let g_exists = unsafe {
            link::H5Lexists(parent, str_to_cchar!(name), plist::H5P_DEFAULT)
        };

        match g_exists.cmp(&0i32) {
            std::cmp::Ordering::Equal => {
                return Err(H5Error::group_doesnt_exists(name));
            }
            std::cmp::Ordering::Less => {
                return Err(H5Error::group_open(name));
            }
            _ => (),
        }

        let gid = unsafe {
            group::H5Gopen2(parent, str_to_cchar!(name), plist::H5P_DEFAULT)
        };
        if gid <= 0 {
            Err(H5Error::group_open(name))
        } else {
            let path_len =
                unsafe { identifier::H5Iget_name(gid, null_mut(), 0) } as usize;
            let buffer = vec![0usize; path_len + 1];
            unsafe {
                identifier::H5Iget_name(gid, buffer.as_ptr() as _, buffer.len())
            };
            let path = unsafe {
                CStr::from_ptr(buffer.as_ptr().cast()).to_str().unwrap()
            };
            Ok(Self { path: path.to_string(), gid })
        }
    }

    pub fn get_gid(&self) -> i64 {
        self.gid
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}

pub struct CreateGroupOptions {
    pub loc_id: types::Hid,
    pub path: String,
    pub link_creation_properties: Option<PList>,
    pub group_creation_properties: Option<PList>,
    pub group_access_properties: Option<PList>,
}

pub trait GroupOpener {
    fn open_group(&self, name: &str) -> Result<Group, H5Error>;
    fn create_group(
        &self,
        options: CreateGroupOptions,
    ) -> Result<Group, H5Error>;
    fn list_groups(&self) -> Vec<String>;
}

impl Drop for Group {
    fn drop(&mut self) {
        if self.gid > 0 {
            #[cfg(debug_assertions)]
            {
                println!("Closing group: {}", self.path);
            }
            unsafe { group::H5Gclose(self.gid) };
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
    fn open_attr(&self, name: &str) -> Result<Attr, H5Error> {
        Attr::open(self.get_gid(), name)
    }
}

impl DatasetOwner for Group {
    fn get_dataset(&self, name: &str) -> Result<DataSet, H5Error> {
        DataSet::open(self.gid, name)
    }

    fn create_dataset(
        &self,
        options: CreateDataSetOptions,
    ) -> Result<DataSet, H5Error> {
        let mut path = self.path.clone();
        path.push('/');
        path.push_str(options.name);

        // checks if the dataset already exists
        if crate::types::link::exists(self.gid, options.name) {
            Err(H5Error::dataset_already_exists(&path))
        } else {
            let did = unsafe {
                dataset::H5Dcreate2(
                    self.gid,
                    str_to_cchar!(options.name),
                    options.dtype.tid,
                    options.dspace.did,
                    match options.link_plist {
                        Some(plist) => plist.pid,
                        None => plist::H5P_DEFAULT,
                    },
                    match options.create_plist {
                        Some(plist) => plist.pid,
                        None => plist::H5P_DEFAULT,
                    },
                    match options.access_plist {
                        Some(plist) => plist.pid,
                        None => plist::H5P_DEFAULT,
                    },
                )
            };

            if did <= 0 {
                Err(H5Error::dataset_creation_failed(&path))
            } else {
                Ok(DataSet {
                    did,
                    path,
                    dataspace: Some(options.dspace),
                    datatype: Some(options.dtype),
                })
            }
        }
    }

    fn list_datasets(&self) -> Vec<String> {
        get_datasets_names(self.get_gid())
    }
}

impl GroupOpener for Group {
    fn open_group(&self, name: &str) -> Result<Group, H5Error> {
        Group::open(self.get_gid(), name)
    }

    fn create_group(
        &self,
        options: CreateGroupOptions,
    ) -> Result<Group, H5Error> {
        // Check if the group already exists
        if exists(options.loc_id, &options.path) {
            Err(H5Error::group_already_exists(&options.path))
        } else {
            Ok(Group {
                path: options.path.clone(),
                gid: {
                    let res = unsafe {
                        group::H5Gcreate2(
                            self.gid,
                            str_to_cchar!(options.path),
                            match options.link_creation_properties {
                                None => plist::H5P_DEFAULT,
                                Some(plist) => plist.pid,
                            },
                            match options.group_creation_properties {
                                None => plist::H5P_DEFAULT,
                                Some(plist) => plist.pid,
                            },
                            match options.group_access_properties {
                                None => plist::H5P_DEFAULT,
                                Some(plist) => plist.pid,
                            },
                        )
                    };
                    if res <= 0 {
                        return Err(H5Error::group_creation_failed(
                            &options.path,
                        ));
                    } else {
                        res
                    }
                },
            })
        }
    }

    fn list_groups(&self) -> Vec<String> {
        get_group_names(self.gid)
    }
}
