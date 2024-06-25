use super::{
    DataSpace, DataSpaceOwner, DataType, DataTypeL, DataTypeOwner, PList,
};
use crate::error::Error;
use crate::h5sys::*;
use crate::{cchar_to_string, str_to_cchar};
use std::any::type_name;

pub struct Dataset {
    did: i64,
    path: String,
    dataspace: Option<DataSpace>,
    datatype: Option<DataType>,
}

pub trait DatasetFillable<T> {
    fn from_dataset(
        _dataset: &Dataset,
        _plist: Option<PList>,
    ) -> Result<Vec<T>, Error> {
        Err(Error::not_yet_implemented(Some(&format!(
                        "Dataset::from_dataset: cannot retrieve the whole dataset from type {}",
                        type_name::<T>()))))
    }

    fn from_dataset_subspace(
        _dataset: &Dataset,
        _start: &[usize],
        _offset: &[usize],
        _plist: Option<PList>,
    ) -> Result<Vec<Vec<T>>, Error> {
        Err(Error::not_yet_implemented(Some(&format!(
                        "Dataset::from_dataset_subspace: cannot retrieve the whole dataset from type {}",
                        type_name::<T>()))))
    }

    fn from_dataset_row(
        _dataset: &Dataset,
        _row: usize,
        _plist: Option<PList>,
    ) -> Result<Vec<T>, Error> {
        Err(Error::not_yet_implemented(Some(&format!(
                        "Dataset::from_dataset_row: cannot retrieve the whole dataset from type {}",
                        type_name::<T>()))))
    }
}

pub trait DatasetOwner {
    fn get_dataset(&self, name: &str) -> Result<Dataset, Error>;
    fn list_datasets(&self) -> Vec<String>;
}

impl Dataset {
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_did(&self) -> i64 {
        self.did
    }

    pub fn get_dataspace(&self) -> Result<&DataSpace, Error> {
        if let Some(ref dataspace) = self.dataspace {
            Ok(dataspace)
        } else {
            Err(Error::dataset_has_no_dataspace(&self.path))
        }
    }

    pub fn get_datatype(&self) -> Result<&DataType, Error> {
        if let Some(ref datatype) = self.datatype {
            Ok(datatype)
        } else {
            Err(Error::dataset_has_no_datatype(&self.path))
        }
    }

    #[allow(unused_unsafe)]
    pub fn open(parent: i64, name: &str) -> Result<Self, Error> {
        let did;
        let path;
        let dataspace;
        let datatype;
        unsafe {
            did = H5Dopen2(parent, str_to_cchar!(name), H5P_DEFAULT);
            if did <= 0 {
                return Err(Error::dataset_open_fail(name));
            }

            let path_len = H5Iget_name(did, null_mut(), 0) as usize;
            let buffer = vec![0usize; path_len + 1];
            H5Iget_name(did, buffer.as_ptr() as _, buffer.len());
            path = cchar_to_string!(buffer.as_ptr().cast());
        }
        let mut ret = Dataset {
            did,
            path,
            dataspace: None,
            datatype: None,
        };
        dataspace = ret.get_space()?;
        datatype = ret.get_type()?;
        ret.dataspace = Some(dataspace);
        ret.datatype = Some(datatype);

        Ok(ret)
    }

    pub fn fill_memory<T>(
        &self,
        memory_datatype: i64,
        data: &mut [T],
    ) -> Result<(), Error> {
        let res;
        unsafe {
            res = H5Dread(
                self.did,
                memory_datatype,
                H5S_ALL,
                H5S_ALL,
                H5P_DEFAULT,
                data.as_ptr() as *mut T as *mut c_void,
            );
        }
        if res >= 0 {
            Ok(())
        } else {
            Err(Error::dataset_fill_memory_fail(&self.path))
        }
    }
}

impl std::fmt::Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5Dataset")?;
        writeln!(f, "  path: {}", self.path)?;
        writeln!(f, "  did {}", self.did)?;
        writeln!(
            f,
            "  dataspace:\n {}",
            if let Some(ref d) = self.dataspace {
                format!("{d}").to_string()
            } else {
                "None".to_string()
            }
        )?;
        writeln!(
            f,
            "  datatype:\n {}",
            if let Some(ref d) = self.datatype {
                format!("{d}").to_string()
            } else {
                "None".to_string()
            }
        )?;
        Ok(())
    }
}

impl Drop for Dataset {
    fn drop(&mut self) {
        if self.did > 0 {
            #[cfg(debug_assertions)]
            {
                println!("Closing dataset {}", self.path);
            }
            unsafe {
                H5Dclose(self.did);
            }
        }
    }
}

impl DataSpaceOwner for Dataset {
    fn get_space(&self) -> Result<DataSpace, Error> {
        DataSpace::parse(unsafe { H5Dget_space(self.did) })
    }
}

impl DataTypeOwner for Dataset {
    fn get_type(&self) -> Result<DataType, Error> {
        DataType::parse(unsafe { H5Dget_type(self.did) })
    }
}

impl DatasetFillable<i32> for i32 {
    fn from_dataset_row(
        dataset: &Dataset,
        row: usize,
        plist: Option<PList>,
    ) -> Result<Vec<i32>, Error> {
        let mut ret = vec![];
        match dataset.get_datatype()?.get_dtype() {
            DataTypeL::Signed32 => {
                let dataspace = dataset.get_dataspace()?;
                let dims = dataspace.get_dims();

                // set subspace
                let memory_dataspace = dataspace.select_row(row)?;

                // create memory dataspace

                ret.resize(dims[1], 0);

                // read the data
                unsafe {
                    H5Dread(
                        dataset.get_did(),
                        H5T_NATIVE_INT_g,
                        memory_dataspace.get_did(),
                        dataspace.get_did(),
                        match plist {
                            Some(plist) => plist.get_pid(),
                            None => H5P_DEFAULT,
                        },
                        ret.as_ptr() as _,
                    )
                };

                // reset original space
                dataspace.reset_selection();
            }
            DataTypeL::Unsigned32 => {
                todo!()
            }
            DataTypeL::Signed64 => {
                todo!()
            }
            DataTypeL::Unsigned64 => {
                todo!()
            }
            _ => {
                return Err(Error::dataset_unvalid_type(
                    &dataset.get_path(),
                    "i32",
                ));
            }
        };

        Ok(ret)
    }
}

impl DatasetFillable<i64> for i64 {
    fn from_dataset_row(
        dataset: &Dataset,
        row: usize,
        plist: Option<PList>,
    ) -> Result<Vec<i64>, Error> {
        let mut ret = vec![];
        match dataset.get_datatype()?.get_dtype() {
            DataTypeL::Signed32 => {
                todo!()
            }
            DataTypeL::Unsigned32 => {
                todo!()
            }
            DataTypeL::Signed64 => {
                let dataspace = dataset.get_dataspace()?;
                let dims = dataspace.get_dims();

                // set subspace
                let memory_dataspace = dataspace.select_row(row)?;

                // create memory dataspace

                ret.resize(dims[1], 0);

                // read the data
                unsafe {
                    H5Dread(
                        dataset.get_did(),
                        H5T_NATIVE_LLONG_g,
                        memory_dataspace.get_did(),
                        dataspace.get_did(),
                        match plist {
                            Some(plist) => plist.get_pid(),
                            None => H5P_DEFAULT,
                        },
                        ret.as_ptr() as _,
                    )
                };

                // reset original space
                dataspace.reset_selection();
            }
            DataTypeL::Unsigned64 => {
                todo!()
            }
            _ => {
                return Err(Error::dataset_unvalid_type(
                    &dataset.get_path(),
                    "i64",
                ));
            }
        };

        Ok(ret)
    }
}

impl DatasetFillable<u64> for u64 {
    fn from_dataset_row(
        dataset: &Dataset,
        row: usize,
        plist: Option<PList>,
    ) -> Result<Vec<u64>, Error> {
        let mut ret = vec![];
        match dataset.get_datatype()?.get_dtype() {
            DataTypeL::Signed32 => {
                todo!()
            }
            DataTypeL::Unsigned32 => {
                todo!()
            }
            DataTypeL::Signed64 => {
                let dataspace = dataset.get_dataspace()?;
                let dims = dataspace.get_dims();

                // set subspace
                let memory_dataspace = dataspace.select_row(row)?;

                // create memory dataspace

                ret.resize(dims[1], 0);

                // read the data
                unsafe {
                    H5Dread(
                        dataset.get_did(),
                        H5T_NATIVE_ULLONG_g,
                        memory_dataspace.get_did(),
                        dataspace.get_did(),
                        match plist {
                            Some(plist) => plist.get_pid(),
                            None => H5P_DEFAULT,
                        },
                        ret.as_ptr() as _,
                    )
                };

                // reset original space
                dataspace.reset_selection();
            }
            DataTypeL::Unsigned64 => {
                todo!()
            }
            _ => {
                return Err(Error::dataset_unvalid_type(
                    &dataset.get_path(),
                    "u64",
                ));
            }
        };

        Ok(ret)
    }
}
