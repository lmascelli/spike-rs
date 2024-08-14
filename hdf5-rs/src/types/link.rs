use crate::{
    str_to_cchar,
    h5sys::{types::Hid, CStr, plist},
    error::H5Error,
};

pub fn exists(loc_id: Hid, name: &str) -> bool {
    return unsafe { crate::h5sys::link::H5Lexists(
        loc_id,
        str_to_cchar!(name),
        plist::H5P_DEFAULT) } > 0
}

pub enum LinkType {
    Dataset,
    Group,
}

#[repr(C)]
pub struct InfoType {

}

pub fn get_link_type(link: Hid) -> Result<LinkType, H5Error> {
    
    todo!()
}
