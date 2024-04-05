use crate::h5sys::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PListType {
    File,
}

pub struct PList {
    ptype: PListType,
    pid: i64,
}

impl PList {
    pub fn get_ptype(&self) -> PListType {
        self.ptype
    }

    pub fn get_pid(&self) -> i64 {
        self.pid
    }
}

impl Drop for PList {
    fn drop(&mut self) {
        if self.pid > 0 {
            #[cfg(debug_assertions)] {
                println!("Closing PList {}", self.pid);
            }
            unsafe { H5Pclose(self.pid); }
        }
    }
}
