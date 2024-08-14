use crate::{error::H5Error, h5sys::*};

#[derive(Clone, Copy, Debug)]
pub enum DataTypeL {
    StringStatic,
    StringDynamic,
    Signed32,
    Unsigned32,
    Signed64,
    Unsigned64,
    Float32,
    Float64,
    Unimplemented,
    Compound,
}

#[derive(Debug)]
pub struct DataType {
    pub tid: i64,
    pub dtype: DataTypeL,
}

pub trait IntoDataType {
    fn into_datatype() -> Result<DataType, H5Error>;
}

impl IntoDataType for i32 {
    fn into_datatype() -> Result<DataType, H5Error> {
        let tid = unsafe { datatype::H5Tcopy(datatype::H5T_NATIVE_INT_g) };
        if tid <= 0 {
            todo!()
        } else {
            Ok(DataType { tid, dtype: DataTypeL::Signed32 })
        }
    }
}

impl IntoDataType for i64 {
    fn into_datatype() -> Result<DataType, H5Error> {
        let tid = unsafe { datatype::H5Tcopy(datatype::H5T_NATIVE_LLONG_g) };
        if tid <= 0 {
            todo!()
        } else {
            Ok(DataType { tid, dtype: DataTypeL::Signed32 })
        }
    }
}

impl IntoDataType for usize {
    fn into_datatype() -> Result<DataType, H5Error> {
        let tid = unsafe { datatype::H5Tcopy(datatype::H5T_NATIVE_ULLONG_g) };
        if tid <= 0 {
            todo!()
        } else {
            Ok(DataType { tid, dtype: DataTypeL::Unsigned64 })
        }
    }
}
impl IntoDataType for f32 {
    fn into_datatype() -> Result<DataType, H5Error> {
        let tid = unsafe { datatype::H5Tcopy(datatype::H5T_NATIVE_FLOAT_g) };
        if tid <= 0 {
            todo!()
        } else {
            Ok(DataType { tid, dtype: DataTypeL::Float32 })
        }
    }
}

impl DataType {
    pub fn open(dtype_id: i64) -> Result<DataType, H5Error> {
        Ok(DataType { tid: dtype_id, dtype: DataType::parse_type(dtype_id)? })
    }

    pub fn get_dtype(&self) -> DataTypeL {
        self.dtype
    }

    #[allow(non_upper_case_globals)]
    pub fn parse_type(dtype_id: i64) -> Result<DataTypeL, H5Error> {
        if dtype_id <= 0 {
            return Err(H5Error::datatype_get_type_fail());
        }
        unsafe {
            match datatype::H5Tget_class(dtype_id) {
                datatype::H5T_class_t_H5T_INTEGER => {
                    let sign = match datatype::H5Tget_sign(dtype_id) {
                        datatype::H5T_sign_t_H5T_SGN_2 => true,
                        datatype::H5T_sign_t_H5T_SGN_NONE => false,
                        _ => {
                            return Ok(DataTypeL::Unimplemented);
                        }
                    };
                    let size = datatype::H5Tget_size(dtype_id);
                    if sign {
                        if size == 4 {
                            return Ok(DataTypeL::Signed32);
                        } else if size == 8 {
                            return Ok(DataTypeL::Signed64);
                        } else {
                            return Ok(DataTypeL::Unimplemented);
                        }
                    }
                    if size == 4 {
                        Ok(DataTypeL::Unsigned32)
                    } else if size == 8 {
                        Ok(DataTypeL::Unsigned64)
                    } else {
                        Ok(DataTypeL::Unimplemented)
                    }
                }

                datatype::H5T_class_t_H5T_FLOAT => {
                    match datatype::H5Tget_size(dtype_id) {
                        4 => Ok(DataTypeL::Float32),
                        8 => Ok(DataTypeL::Float64),
                        _ => Ok(DataTypeL::Unimplemented),
                    }
                }

                datatype::H5T_class_t_H5T_STRING => {
                    let padding = datatype::H5Tget_strpad(dtype_id);
                    let cset = datatype::H5Tget_cset(dtype_id);
                    let is_variable = {
                        let r = datatype::H5Tis_variable_str(dtype_id);
                        match r {
                            0 => false,
                            _ if r > 0 => true,
                            _ => {
                                return Err(
                                    H5Error::datatype_parse_string_is_variable(
                                    ),
                                );
                            }
                        }
                    };

                    if padding == datatype::H5T_str_t_H5T_STR_NULLTERM
                        && cset == datatype::H5T_cset_t_H5T_CSET_ASCII
                    {
                        Ok(if is_variable {
                            DataTypeL::StringDynamic
                        } else {
                            DataTypeL::StringStatic
                        })
                    } else {
                        Err(H5Error::datatype_parse_string_type_not_supported())
                    }
                }

                datatype::H5T_class_t_H5T_COMPOUND => Ok(DataTypeL::Compound),
                _ => Ok(DataTypeL::Unimplemented),
            }
        }
    }

    pub fn get_tid(&self) -> i64 {
        self.tid
    }

    pub fn size(&self) -> usize {
        unsafe { datatype::H5Tget_size(self.tid) }
    }
}

impl Drop for DataType {
    fn drop(&mut self) {
        if self.tid > 0 {
            #[cfg(debug_assertions)]
            {
                println!("Closing type: {}", self.tid);
            }
            unsafe { datatype::H5Tclose(self.tid) };
        }
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5Type")?;
        writeln!(
            f,
            "  type: {}",
            match self.dtype {
                DataTypeL::Signed32 => "Signed32",
                DataTypeL::Signed64 => "Signed64",
                DataTypeL::Unsigned32 => "Unsigned32",
                DataTypeL::Unsigned64 => "Unsigned64",
                DataTypeL::Float32 => "Float32",
                DataTypeL::Float64 => "Float64",
                DataTypeL::StringStatic => "String static",
                DataTypeL::StringDynamic => "String dynamic",
                DataTypeL::Compound => "Compound datatype",
                DataTypeL::Unimplemented => "Unimplemented type conversion",
            }
        )?;
        writeln!(f, "  tid: {}", self.tid)?;
        writeln!(f, "  size: {}", self.size())?;
        Ok(())
    }
}

pub trait DataTypeOwner {
    fn get_type(&self) -> Result<DataType, H5Error>;
}
