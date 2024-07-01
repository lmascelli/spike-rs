use crate::{error::H5Error, h5sys::*, str_to_cchar};

pub type CFilter = extern "C" fn(
    flags: u32,
    cd_nelmts: usize,
    cd_values: *const u32,
    nbytes: usize,
    buf_size: *mut usize,
    buf: *mut *mut core::ffi::c_void,
) -> usize;

pub struct Filter {
    pub fid: i32,
    pub desc: String,
    pub n_params: usize,
    pub flags: u32,
    pub cls: Option<filter::H5Z_class2_t>,
}

const HDF5_FILTER_COUNTER: std::cell::Cell<i32> = std::cell::Cell::new(512);

impl Filter {
    pub fn get_next_fid() -> i32 {
        let ret = HDF5_FILTER_COUNTER.get();
        HDF5_FILTER_COUNTER.set(ret + 1);
        ret
    }

    pub fn create(
        function: CFilter,
        n_params: usize,
        desc: Option<&str>,
    ) -> Result<Self, H5Error> {
        let mut ret = Self {
            fid: Self::get_next_fid(),
            desc: desc.unwrap_or("").to_string(),
            n_params: 0,
            flags: 0,
            cls: None,
        };
        ret.set_cls(function, n_params);
        unsafe {
            if filter::H5Zregister(
                &ret.cls.unwrap() as *const _ as *const c_void
            ) < 0
            {
                Err(H5Error::filter_registration_failed(
                    desc.unwrap_or("Unnamed filter"),
                ))
            } else {
                Ok(ret)
            }
        }
    }

    pub fn get_fid(&self) -> i32 {
        self.fid
    }

    pub fn set_cls(
        &mut self,
        filter_function: CFilter,
        filter_n_params: usize,
    ) {
        self.n_params = filter_n_params;
        self.cls = Some(filter::H5Z_class2_t {
            version: filter::H5Z_CLASS_T_VERS as i32,
            id: self.fid,
            encoder_present: 0,
            decoder_present: 0,
            name: str_to_cchar!(&self.desc),
            can_apply: None,
            set_local: None,
            filter: Some(filter_function),
        });
    }
}

impl Drop for Filter {
    fn drop(&mut self) {
        unsafe {
            filter::H5Zunregister(self.fid);
        }
    }
}
