use crate::{
    error::H5Error,
    h5sys::{plist, types::Hid, CStr},
    link, str_to_cchar,
};

pub fn exists(loc_id: Hid, name: &str) -> bool {
    return unsafe {
        crate::h5sys::link::H5Lexists(
            loc_id,
            str_to_cchar!(name),
            plist::H5P_DEFAULT,
        )
    } > 0;
}

#[derive(Default, Debug)]
pub enum LinkType {
    #[default]
    Unknown,
    Group,
    Dataset,
    NamedDatatype,
    Map,
    Ntypes,
}

pub fn get_link_type(link: Hid) -> Result<LinkType, H5Error> {
    let mut link_info = link::H5O_info2_t {
        fileno: 0,
        token: link::H5O_token_t { __data: [0u8; 16] },
        type_: 0,
        rc: 0,
        atime: 0,
        mtime: 0,
        ctime: 0,
        btime: 0,
        num_attrs: 0,
    };

    if unsafe {
        link::H5Oget_info3(link, &mut link_info as *mut link::H5O_info2_t, 1)
    } < 0 {
        Err(H5Error::link_get_type_failed())
    } else {
        Ok(match link_info.type_ {
            -1=> LinkType::Unknown,
            0 => LinkType::Group,
            1 => LinkType::Dataset,
            2 => LinkType::NamedDatatype,
            3 => LinkType::Map,
            _ => {
                println!("ERROR: get_link_type unvalid type {}", link_info.type_);
                todo!()
            }
        })
    }

}
