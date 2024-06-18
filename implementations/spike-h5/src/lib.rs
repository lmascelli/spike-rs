use std::error::Error;

use hdf5_rs::{
    types::{File, FileOpenAccess},
};

use spike_rs::error;

#[derive(Debug)]
pub struct PhaseH5 {
    
}

impl PhaseH5 {
    pub fn open(filename: &str) -> Result<Self, error::SpikeError> {
        Ok(Self {})
    }
}

fn f() -> Result<PhaseH5, impl Error> {
    PhaseH5::open("")
}
