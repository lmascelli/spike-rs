use super::{
    DataSpace, DataSpaceOwner, DataType, DataTypeL, DataTypeOwner, PList,
};
use crate::{error::Error, h5sys::*, Hdf5};
use std::any::type_name;

pub struct Dataset<'lib> {
    pub lib: &'lib Hdf5,
    pub did: i64,
    pub path: String,
    pub dataspace: Option<DataSpace<'lib>>,
    pub datatype: Option<DataType<'lib>>,
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

impl<'lib> Dataset<'lib> {
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

impl<'lib> std::fmt::Display for Dataset<'lib> {
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

impl<'lib> Drop for Dataset<'lib> {
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

impl<'lib> DataSpaceOwner for Dataset<'lib> {
    fn get_space(&self) -> Result<DataSpace, Error> {
        self.lib.open_dataspace(unsafe { H5Dget_space(self.did) })
    }
}

impl<'lib> DataTypeOwner for Dataset<'lib> {
    fn get_type(&self) -> Result<DataType, Error> {
        self.lib.open_type(unsafe { H5Dget_type(self.did) })
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
