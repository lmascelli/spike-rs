use crate::error::H5Error;
use crate::h5sys::*;

pub fn is_valid_id(id: types::Hid) -> bool {
    let res = unsafe { identifier::H5Iis_valid(id) };
    if res <= 0 {
        false
    } else {
        true
    }
}

pub fn check_id(id: types::Hid) -> Result<(), H5Error> {
    if is_valid_id(id) {
        Ok(())
    } else {
        Err(H5Error::id_unvalid())
    }
}
