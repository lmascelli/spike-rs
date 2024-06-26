use crate::{error::Error, h5sys::*, Hdf5};

pub enum DataSpaceType {
    Null,
    Scalar,
    Simple,
}

pub struct DataSpace<'lib> {
    pub lib: &'lib Hdf5,
    pub did: i64,
    pub space_type: DataSpaceType,
    pub dims: Vec<usize>,
}

impl<'lib> DataSpace<'lib> {
    pub fn get_did(&self) -> i64 {
        self.did
    }

    pub fn get_dims(&self) -> &[usize] {
        &self.dims[..]
    }

    pub fn select_slab(
        &self,
        start: &[u64],
        offset: &[u64],
    ) -> Result<DataSpace, Error> {
        if start.len() != self.dims.len() || start.len() != offset.len() {
            Err(Error::dataspace_select_slab_fail(
                start,
                offset,
                &self.dims[..]
                    .iter()
                    .map(|x| *x as u64)
                    .collect::<Vec<u64>>(),
            ))
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

                    Ok(self.lib.open_dataspace(H5Screate_simple(
                        offset.len() as i32,
                        offset.as_ptr(),
                        null(),
                    ))?)
                }
            } else {
                Err(Error::dataspace_select_slab_out_of_boulds(
                    start,
                    offset,
                    &self.dims[..]
                        .iter()
                        .map(|x| *x as u64)
                        .collect::<Vec<u64>>(),
                ))
            }
        }
    }

    pub fn select_row(&self, row: usize) -> Result<DataSpace, Error> {
        if self.dims.len() != 2 {
            Err(Error::dataspace_select_row_not_bidimensional(
                &self.dims[..]
                    .iter()
                    .map(|x| *x as u64)
                    .collect::<Vec<u64>>(),
            ))
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

impl<'lib> Drop for DataSpace<'lib> {
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

impl<'lib> std::fmt::Display for DataSpace<'lib> {
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
