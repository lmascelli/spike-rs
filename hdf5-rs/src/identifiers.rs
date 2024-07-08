use crate::error::H5Error;
use crate::{cchar_to_string, h5sys::*};

pub enum HidType {
    Unitialized,
    Invalid,
    File,
    Group,
    DataType,
    DataSpace,
    DataSet,
    Map,
    Attibute,
    VirtualFileLayer,
    VirtualObjectLayer,
    GenericPListClass,
    GenericPList,
    ErrorClass,
    ErrorMessage,
    ErrorStack,
    DataSpaceSelectionIterator,
    EventSet,
}

pub type HidTypeId = i32;

impl HidType {
    pub fn get_id(&self) -> HidTypeId {
        match self {
            HidType::Unitialized => identifier::H5I_type_t_H5I_UNINIT,
            HidType::Invalid => identifier::H5I_type_t_H5I_BADID,
            HidType::File => identifier::H5I_type_t_H5I_FILE,
            HidType::Group => identifier::H5I_type_t_H5I_GROUP,
            HidType::DataType => identifier::H5I_type_t_H5I_DATATYPE,
            HidType::DataSpace => identifier::H5I_type_t_H5I_DATASPACE,
            HidType::DataSet => identifier::H5I_type_t_H5I_DATASET,
            HidType::Map => identifier::H5I_type_t_H5I_MAP,
            HidType::Attibute => identifier::H5I_type_t_H5I_ATTR,
            HidType::VirtualFileLayer => identifier::H5I_type_t_H5I_VFL,
            HidType::VirtualObjectLayer => identifier::H5I_type_t_H5I_VOL,
            HidType::GenericPListClass => {
                identifier::H5I_type_t_H5I_GENPROP_CLS
            }
            HidType::GenericPList => identifier::H5I_type_t_H5I_GENPROP_LST,
            HidType::ErrorClass => identifier::H5I_type_t_H5I_ERROR_CLASS,
            HidType::ErrorMessage => identifier::H5I_type_t_H5I_ERROR_MSG,
            HidType::ErrorStack => identifier::H5I_type_t_H5I_ERROR_STACK,
            HidType::DataSpaceSelectionIterator => {
                identifier::H5I_type_t_H5I_SPACE_SEL_ITER
            }
            HidType::EventSet => identifier::H5I_type_t_H5I_EVENTSET,
        }
    }
}

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

pub fn get_name(id: types::Hid) -> String {
    let mut name_size = 16;
    let actual_name_size;
    let mut name = vec![0i8; name_size];
    unsafe {
        actual_name_size =
            identifier::H5Iget_name(id, name.as_mut_ptr(), name_size);
    }

    if actual_name_size as usize > name_size {
        name_size = actual_name_size as usize;
        name = vec![0i8; name_size];

        unsafe {
            identifier::H5Iget_name(id, name.as_mut_ptr(), name_size);
        }
    }

    cchar_to_string!(name.as_ptr())
}

pub fn inc_ref(id: types::Hid) -> i32 {
    unsafe { identifier::H5Iinc_ref(id) }
}

pub fn dec_ref(id: types::Hid) -> i32 {
    unsafe { identifier::H5Idec_ref(id) }
}

pub fn get_ref(id: types::Hid) -> i32 {
    unsafe { identifier::H5Iget_ref(id) }
}
