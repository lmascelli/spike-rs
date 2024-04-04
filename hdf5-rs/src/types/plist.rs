use crate::h5sys::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PListType {
    File,
}

pub struct PList {
    ptype: PListType,
    pid: Option<i64>,
}

impl PList {
    pub fn get_ptype(&self) -> PListType {
        self.ptype
    }

    pub fn get_pid(&self) -> Option<i64> {
        self.pid
    }
}

impl Default for PList {
    fn default() -> Self {
        let pid = unsafe {H5Pcopy(H5P_DEFAULT)};
        PList {
            ptype: PListType::File,
            pid: if pid > 0 {
                Some(pid)
            } else {
                None
            },
        }
    }
}
