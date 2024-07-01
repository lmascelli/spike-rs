use super::{
    DataSpace, DataSpaceOwner, DataType, DataTypeL, DataTypeOwner, PList,
    PListClass,
};
use crate::{
    cchar_to_string, error::H5Error, h5sys::*, identifiers::check_id,
    str_to_cchar,
};
use std::any::type_name;

pub struct DataSet {
    pub did: i64,
    pub path: String,
    pub dataspace: Option<DataSpace>,
    pub datatype: Option<DataType>,
}

pub trait DatasetFillable<T> {
    fn from_dataset(
        _dataset: &DataSet,
        _plist: Option<&PList>,
    ) -> Result<Vec<T>, H5Error> {
        Err(H5Error::not_yet_implemented(Some(&format!(
                        "Dataset::from_dataset: cannot retrieve the whole dataset from type {}",
                        type_name::<T>()))))
    }

    fn from_dataset_subspace(
        _dataset: &DataSet,
        _start: &[usize],
        _offset: &[usize],
        _plist: Option<&PList>,
    ) -> Result<Vec<Vec<T>>, H5Error> {
        Err(H5Error::not_yet_implemented(Some(&format!(
                        "Dataset::from_dataset_subspace: cannot retrieve the whole dataset from type {}",
                        type_name::<T>()))))
    }

    fn from_dataset_row(
        _dataset: &DataSet,
        _row: usize,
        _plist: Option<&PList>,
    ) -> Result<Vec<T>, H5Error> {
        Err(H5Error::not_yet_implemented(Some(&format!(
                        "Dataset::from_dataset_row: cannot retrieve the whole dataset from type {}",
                        type_name::<T>()))))
    }
}

pub trait DatasetOwner {
    fn get_dataset(&self, name: &str) -> Result<DataSet, H5Error>;
    fn list_datasets(&self) -> Vec<String>;
}

impl DataSet {
    #[allow(unused_unsafe)]
    pub fn open(parent: i64, name: &str) -> Result<Self, H5Error> {
        let did;
        let path;
        let dataspace_id;
        let datatype_id;
        unsafe {
            did = dataset::H5Dopen2(parent, str_to_cchar!(name), plist::H5P_DEFAULT);
            if did <= 0 {
                return Err(H5Error::dataset_open_fail(name));
            }
            let path_len = identifier::H5Iget_name(did, null_mut(), 0) as usize;
            let buffer = vec![0usize; path_len + 1];
            identifier::H5Iget_name(did, buffer.as_ptr() as _, buffer.len());
            path = cchar_to_string!(buffer.as_ptr().cast());

            dataspace_id = dataset::H5Dget_space(did);
            if dataspace_id <= 0 {
                return Err(H5Error::dataset_has_no_dataspace(&path));
            }
            datatype_id = dataset::H5Dget_type(did);
            if datatype_id <= 0 {
                return Err(H5Error::dataset_has_no_datatype(&path));
            }
        }

        Ok(Self {
            did,
            path,
            dataspace: Some(DataSpace::open(dataspace_id)?),
            datatype: Some(DataType::open(datatype_id)?),
        })
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_did(&self) -> i64 {
        self.did
    }

    pub fn get_dataspace(&self) -> Result<&DataSpace, H5Error> {
        if let Some(ref dataspace) = self.dataspace {
            Ok(dataspace)
        } else {
            Err(H5Error::dataset_has_no_dataspace(&self.path))
        }
    }

    pub fn get_datatype(&self) -> Result<&DataType, H5Error> {
        if let Some(ref datatype) = self.datatype {
            Ok(datatype)
        } else {
            Err(H5Error::dataset_has_no_datatype(&self.path))
        }
    }

    pub fn copy_create_plist(&self) -> Result<PList, H5Error> {
        unsafe {
            let pid = dataset::H5Dget_create_plist(self.did);
            check_id(pid)?;
            let class = PListClass::from_id(pid);
            let pid_copy = plist::H5Pcopy(pid);
            check_id(pid_copy)?;
            let ret = PList { pid: pid_copy, class };
            ret.check_class(PListClass::DataSetCreate)?;
            Ok(ret)
        }
    }

    pub fn get_chunk(&self) -> Result<Option<Vec<usize>>, H5Error> {
        let cplist = self.copy_create_plist()?;
        let dims = self.get_dataspace()?.get_dims().len();
        let chunk_sizes = vec![0; dims];
        if unsafe {
            plist::H5Pget_chunk(
                cplist.pid,
                dims as i32,
                chunk_sizes.as_ptr().cast_mut(),
            )
        } >= 0
        {
            Ok(Some(chunk_sizes.iter().map(|x| *x as usize).collect()))
        } else {
            Ok(None)
        }
    }

    pub fn fill_memory<T>(
        &self,
        memory_datatype: i64,
        data: &mut [T],
    ) -> Result<(), H5Error> {
        let res;
        unsafe {
            res = dataset::H5Dread(
                self.did,
                memory_datatype,
                dataspace::H5S_ALL,
                dataspace::H5S_ALL,
                plist::H5P_DEFAULT,
                data.as_ptr() as *mut T as *mut c_void,
            );
        }
        if res >= 0 {
            Ok(())
        } else {
            Err(H5Error::dataset_fill_memory_fail(&self.path))
        }
    }
}

impl std::fmt::Display for DataSet {
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

impl Drop for DataSet {
    fn drop(&mut self) {
        if self.did > 0 {
            #[cfg(debug_assertions)]
            {
                println!("Closing dataset {}", self.path);
            }
            unsafe {
                dataset::H5Dclose(self.did);
            }
        }
    }
}

impl DataSpaceOwner for DataSet {
    fn get_space(&self) -> Result<DataSpace, H5Error> {
        DataSpace::open(unsafe { dataset::H5Dget_space(self.did) })
    }
}

impl DataTypeOwner for DataSet {
    fn get_type(&self) -> Result<DataType, H5Error> {
        DataType::open(unsafe { dataset::H5Dget_type(self.did) })
    }
}

impl DatasetFillable<i32> for i32 {
    fn from_dataset_row(
        dataset: &DataSet,
        row: usize,
        plist: Option<&PList>,
    ) -> Result<Vec<i32>, H5Error> {
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
                    dataset::H5Dread(
                        dataset.get_did(),
                        datatype::H5T_NATIVE_INT_g,
                        memory_dataspace.get_did(),
                        dataspace.get_did(),
                        match plist {
                            Some(plist) => plist.get_pid(),
                            None => plist::H5P_DEFAULT,
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
                return Err(H5Error::dataset_unvalid_type(
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
        dataset: &DataSet,
        row: usize,
        plist: Option<&PList>,
    ) -> Result<Vec<i64>, H5Error> {
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
                    dataset::H5Dread(
                        dataset.get_did(),
                        datatype::H5T_NATIVE_LLONG_g,
                        memory_dataspace.get_did(),
                        dataspace.get_did(),
                        match plist {
                            Some(plist) => plist.get_pid(),
                            None => plist::H5P_DEFAULT,
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
                return Err(H5Error::dataset_unvalid_type(
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
        dataset: &DataSet,
        row: usize,
        plist: Option<&PList>,
    ) -> Result<Vec<u64>, H5Error> {
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
                    dataset::H5Dread(
                        dataset.get_did(),
                        datatype::H5T_NATIVE_ULLONG_g,
                        memory_dataspace.get_did(),
                        dataspace.get_did(),
                        match plist {
                            Some(plist) => plist.get_pid(),
                            None => plist::H5P_DEFAULT,
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
                return Err(H5Error::dataset_unvalid_type(
                    &dataset.get_path(),
                    "u64",
                ));
            }
        };

        Ok(ret)
    }
}
