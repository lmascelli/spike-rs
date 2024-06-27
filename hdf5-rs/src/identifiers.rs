use crate::error::Error;
use crate::h5sys::*;

pub fn check_id(id: Hid) -> Result<(), Error> {
    let res = unsafe { H5Iis_valid(id) };
    if res <= 0 {
        Err(Error::id_unvalid())
    } else {
        Ok(())
    }
}
