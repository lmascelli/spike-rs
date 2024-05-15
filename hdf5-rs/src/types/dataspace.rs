use crate::h5sys::*;
use crate::error::Error;

pub enum DataSpaceType {
    Null,
    Scalar,
    Simple,
}

pub struct DataSpace {
    did: i64,
    space_type: DataSpaceType,
    dims: Vec<usize>,
}

impl DataSpace {
    pub fn new_simple(dims: &[u64]) -> Result<DataSpace, Error> {
        let did = unsafe { H5Screate_simple(dims.len() as i32, dims.as_ptr(), null()) };
        if did > 0 {
            Ok(DataSpace {
                did,
                space_type: DataSpaceType::Simple,
                dims: dims.iter().map(|x| *x as usize).collect(),
            })
        } else {
            Err(Error::dataspace_simple_new(dims))
        }
    }

    pub fn parse(dataspace_id: i64) -> Result<Self, Error> {
        let n_dims;
        let mut dims = vec![];
        let space_type;
        unsafe {
            n_dims = H5Sget_simple_extent_ndims(dataspace_id);
            if n_dims < 0 {
                return Err(Error::dataspace_get_dimensions_fail());
            }
            dims.resize(n_dims as usize, 0usize);
            space_type = if n_dims == 0 {
                DataSpaceType::Scalar
            } else {
                DataSpaceType::Simple
            };
            H5Sget_simple_extent_dims(dataspace_id, dims.as_ptr().cast_mut().cast(), null_mut());
        }

        Ok(DataSpace {
            did: dataspace_id,
            space_type,
            dims,
        })
    }

    pub fn get_did(&self) -> i64 {
        self.did
    }

    pub fn get_dims(&self) -> &[usize] {
        &self.dims[..]
    }

    pub fn select_slab(&self, start: &[u64], offset: &[u64]) -> Result<DataSpace, Error> {
        if start.len() != self.dims.len() || start.len() != offset.len() {
            Err(Error::dataspace_select_slab_fail(start, offset, &self.dims[..].iter().map(|x| *x as u64).collect::<Vec<u64>>()))
        } else {
            let mut valid = true;
            for dim in 0..start.len() {
                if start[dim] as usize >= self.dims[dim]
                    || (start[dim] + offset[dim]) as usize > self.dims[dim]
                {
                    valid = false;
                    break;
                }
            }
            if valid {
                unsafe {
                    H5Sselect_hyperslab(
                        self.did,
                        H5S_seloper_t_H5S_SELECT_SET,
                        start.as_ptr(),
                        null(),
                        offset.as_ptr(),
                        null(),
                    );

                    Ok(DataSpace::parse(H5Screate_simple(
                        offset.len() as i32,
                        offset.as_ptr(),
                        null(),
                    ))?)
                }
            } else {
                Err(Error::dataspace_select_slab_out_of_boulds(start, offset, &self.dims[..].iter().map(|x| *x as u64).collect::<Vec<u64>>()))
            }
        }
    }

    pub fn select_row(&self, row: usize) -> Result<DataSpace, Error> {
        if self.dims.len() != 2 {
            Err(Error::dataspace_select_row_not_bidimensional(&self.dims[..].iter().map(|x| *x as u64).collect::<Vec<u64>>()))
        } else {
            let start = [row as u64, 0];
            let offset = [1, (self.dims[1]) as u64];
            self.select_slab(&start[..], &offset[..])
        }
    }

    pub fn reset_selection(&self) {
        unsafe {
            H5Sselect_all(self.did);
        }
    }
}

impl Drop for DataSpace {
    fn drop(&mut self) {
        if self.did > 0 {
            #[cfg(debug_assertions)]
            {
                println!("Closing dataspace: {}", self.did);
            }
            unsafe {
                H5Sclose(self.did);
            }
        }
    }
}

impl std::fmt::Display for DataSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5DataSpace")?;
        writeln!(f, "  did: {}", self.did)?;
        match self.space_type {
            DataSpaceType::Null => {
                writeln!(f, "  dims: Null")?;
            }
            DataSpaceType::Scalar => {
                writeln!(f, "  dims: Scalar")?;
            }
            DataSpaceType::Simple => {
                writeln!(f, "  dims: {:?}", self.dims)?;
            }
        };
        Ok(())
    }
}

pub trait DataSpaceOwner {
    fn get_space(&self) -> Result<DataSpace, Error>;
}
