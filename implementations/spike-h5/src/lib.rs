use hdf5_rs::types::{File, FileOpenAccess};

use spike_rs::error::SpikeError;
use hdf5_rs::error::Error as H5Error;

pub enum PhaseH5Error {
    SpikeError(SpikeError),
    H5Error(H5Error),
}

impl From<H5Error> for PhaseH5Error {
    fn from(value: H5Error) -> Self {
        PhaseH5Error::H5Error(value)
    }
}

impl From<SpikeError> for PhaseH5Error {
    fn from(value: SpikeError) -> Self {
        PhaseH5Error::SpikeError(value)
    }
}

pub struct PhaseH5 {
    file: File,
}

impl PhaseH5 {
    pub fn open(filename: &str) -> Result<Self, PhaseH5Error> {
        let file = File::open(filename, FileOpenAccess::ReadWrite)?;
        Ok(Self {
            file
        })
    }
}
