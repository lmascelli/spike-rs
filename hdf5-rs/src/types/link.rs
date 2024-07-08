use crate::{
    str_to_cchar,
    h5sys::{types::Hid, CStr, plist}
};

pub fn exists(loc_id: Hid, name: &str) -> bool {
    return unsafe { crate::h5sys::link::H5Lexists(
        loc_id,
        str_to_cchar!(name),
        plist::H5P_DEFAULT) } > 0
}
