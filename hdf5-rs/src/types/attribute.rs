use crate::h5sys::*;
use crate::utils::*;
use crate::types::{DataSpace, DataSpaceOwner, DataType, DataTypeL, DataTypeOwner};

pub struct Attr {
    name: String,
    aid: i64,
    datatype: DataType,
}

pub trait AttributeFillable<T> {
    fn from_attribute(attribute: &Attr) -> Result<T, String>;
}

impl Attr {
    pub fn open(group: i64, name: &str) -> Result<Self, String> {
        let aid;
        let atype_id;
        unsafe {
            aid = H5Aopen(group, str_to_cchar!(name), H5P_DEFAULT);
            if aid <=0 {
                return Err(format!("Failed to open attribute {}", name));
            }
            atype_id = H5Aget_type(aid);
            if atype_id <= 0 {
                return Err(format!("Failed to retrieve the type of the attribute {}", name));
            }
        }
        
        Ok(Attr {
            name: name.to_string(),
            aid,
            datatype: DataType::parse(atype_id),
        })
    }

    pub fn get_aid(&self) -> i64 {
        self.aid
    }

    pub fn get_datatype(&self) -> &DataType {
        &self.datatype
    }
}

impl Drop for Attr {
    fn drop(&mut self) {
        if self.aid > 0 {
            println!("Closing attribute: {}", self.aid);
            unsafe { H5Aclose(self.aid); }
        }
    }
}

impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5Attribute")?;
        writeln!(f, "  name: {}", self.name)?;
        writeln!(f, "  aid: {}", self.aid)?;
        write!(f, "  value: ")?;
        match self.datatype.get_dtype() {
            DataTypeL::StringStatic => {
                writeln!(f, "{}", String::from_attribute(self).unwrap())?;
            }
            DataTypeL::Signed64 => {
                writeln!(f, "{}", i64::from_attribute(self).unwrap())?;
            }
            _ => { writeln!(f, "not yet implemented")?; }
        }
        Ok(())
    }
}

pub trait AttrOpener {
    fn open_attr(&self, name: &str) -> Result<Attr, String>;
}

impl DataTypeOwner for Attr {
    fn get_type(&self) -> Result<DataType, String> {
        let dtype_id = unsafe { H5Aget_type(self.aid) };
        if dtype_id <= 0 {
            Err(format!("Dataset::get_space: Failed to retrieve the DataType for {} attribute", self.name))
        } else {
            Ok(DataType::parse(dtype_id))
        }
    }
}

impl DataSpaceOwner for Attr {
    fn get_space(&self) -> Result<DataSpace, String> {
        let did = unsafe { H5Aget_space(self.get_aid()) };
        if did <= 0 {
            return Err("Attr::get_space: failed to retrieve the attribute dataspace".to_string());
        }
        DataSpace::parse(did)
    }
}

impl AttributeFillable<String> for String {
    fn from_attribute(attribute: &Attr) -> Result<String, String> {
        let data_type = attribute.get_datatype();
        match data_type.get_dtype() {
            DataTypeL::StringStatic => {
                let mut data = vec![0; data_type.size()];
                unsafe {
                    H5Aread(
                        attribute.get_aid(),
                        data_type.get_tid(),
                        data
                        .as_mut_ptr()
                        .cast()
                    );
                    return if let Ok(s) = CStr::from_ptr(data.as_ptr().cast()).to_str() {
                        Ok(s.to_string())
                    } else {
                        Err("Failed to convert the attribute data".to_string())
                    }

                }
            },
            DataTypeL::StringDynamic => {
                todo!("Dynamic string not yet implemented")
            },
            _ => {
                Err("Attribute::from_attribute: cannot extract a string from this attribute".to_string())
            }
        }
    }
}

impl AttributeFillable<i64> for i64 {
    fn from_attribute(attribute: &Attr) -> Result<i64, String> {
        let data_type = attribute.get_datatype();
        match data_type.get_dtype() {
            DataTypeL::Signed64 => {
                let mut data = 0i64;
                unsafe {
                    H5Aread(
                        attribute.get_aid(),
                        data_type.get_tid(),
                        &mut data as *mut i64 as *mut c_void
                    );
                    Ok(data)
                }
            },
            _ => {
                Err("Attribute::from_attribute: cannot extract a string from this attribute".to_string())
            }
        }
    }
}