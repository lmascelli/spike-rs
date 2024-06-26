use crate::Hdf5;

pub type CFilter = extern "C" fn(
    flags: u32,
    cd_nelmts: usize,
    cd_values: *const u32,
    nbytes: usize,
    buf_size: *mut usize,
    buf: *mut *mut core::ffi::c_void,
) -> usize;

pub struct Filter<'lib> {
    pub lib: &'lib Hdf5,
    pub fid: i32,
    pub desc: String,
    pub function: CFilter,
    pub n_params: usize,
    pub flags: u32,
}

impl<'lib> Filter<'lib> {
    pub fn get_fid(&self) -> i32 {
        self.fid
    }
}
