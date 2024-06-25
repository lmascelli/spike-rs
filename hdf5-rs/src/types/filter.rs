pub type CFilter = extern "C" fn(
    flags: u32,
    cd_nelmts: usize,
    cd_values: *const u32,
    nbytes: usize,
    buf_size: *mut usize,
    buf: *mut *mut core::ffi::c_void,
) -> usize;

pub struct FilterRuntime {
    filter_counter: i32,
}

impl FilterRuntime {
    pub fn new() -> Self {
        Self {
            filter_counter: 512,
        }
    }

    pub fn next(&mut self) -> i32 {
        let ret = self.filter_counter;
        self.filter_counter += 1;
        ret
    }
}

pub struct Filter {
    fid: i32,
    pub desc: String,
    pub function: CFilter,
    pub n_params: usize,
    pub flags: u32,
}

impl Filter {
    pub fn new(
        fruntime: &mut FilterRuntime,
        function: CFilter,
        desc: Option<&str>,
    ) -> Self {
        Self {
            fid: fruntime.next(),
            function,
            desc: desc.unwrap_or("").to_string(),
            n_params: 0,
            flags: 0,
        }
    }

    pub fn get_fid(&self) -> i32 {
        self.fid
    }
}
