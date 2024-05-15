use crate::h5sys::*;
use crate::error::Error;

#[derive(Clone, Copy)]
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

pub struct DataType {
    tid: i64,
    dtype: DataTypeL,
}

impl DataType {
    pub fn get_dtype(&self) -> DataTypeL {
        self.dtype
    }
    
    #[allow(non_upper_case_globals)]
    pub fn parse(dtype_id: i64) -> Result<Self, Error> {
        if dtype_id <= 0 {
            return Err(Error::datatype_get_type_fail());
        }
        unsafe {
            match H5Tget_class(dtype_id) {
                H5T_class_t_H5T_INTEGER => {
                    let sign = match H5Tget_sign(dtype_id) {
                        H5T_sign_t_H5T_SGN_2 => true,
                        H5T_sign_t_H5T_SGN_NONE => false,
                        _ => {
                            return Ok(DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Unimplemented,
                            });
                        }
                    };
                    let size = H5Tget_size(dtype_id);
                    if sign {
                        if size == 4 {
                            return Ok(DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Signed32,
                            });
                        }
                        else if size == 8 {
                            return Ok(DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Signed64,
                            });
                        } else {
                            return Ok(DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Unimplemented,
                            });
                        }
                    }
                    if size == 4 {
                        Ok(DataType {
                            tid: dtype_id,
                            dtype: DataTypeL::Unsigned32,
                        })
                    }
                    else if size == 8 {
                        Ok(DataType {
                            tid: dtype_id,
                            dtype: DataTypeL::Unsigned64,
                        })
                    } else {
                        Ok(DataType {
                            tid: dtype_id,
                            dtype: DataTypeL::Unimplemented,
                        })
                    }
                },

                H5T_class_t_H5T_FLOAT => {
                    match H5Tget_size(dtype_id) {
                        4 => {
                            Ok(DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Float32,
                            })
                        },
                        8 => {
                            Ok(DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Float64,
                            })
                        },
                        _ => {
                            Ok(DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Unimplemented,
                            })
                        },
                    }
                },

                H5T_class_t_H5T_STRING => {
                    let padding = H5Tget_strpad(dtype_id);
                    let cset = H5Tget_cset(dtype_id);
                    let is_variable = {
                        let r = H5Tis_variable_str(dtype_id);
                        match r {
                            0   => false,
                            _ if r > 0 => true,
                            _   => {
                                return Err(Error::datatype_parse_string_is_variable());
                            },
                        }
                    };

                    if padding == H5T_str_t_H5T_STR_NULLTERM && cset == H5T_cset_t_H5T_CSET_ASCII {
                        Ok(DataType {
                            tid: dtype_id,
                            dtype: if is_variable {
                                DataTypeL::StringDynamic
                            } else {
                                DataTypeL::StringStatic
                            },
                        })
                    } else {
                        Err(Error::datatype_parse_string_type_not_supported())
                    }
                },

                H5T_class_t_H5T_COMPOUND => {
                        Ok(DataType {
                            tid: dtype_id,
                            dtype: DataTypeL::Compound,
                        })
                },
                _ => {
                    Ok(DataType {
                        tid: dtype_id,
                        dtype: DataTypeL::Unimplemented,
                    })
                }
            }
        }
    }

    pub fn get_tid(&self) -> i64 {
        self.tid
    }

    pub fn size(&self) -> usize {
        unsafe { H5Tget_size(self.tid) }
    }
}

impl Drop for DataType {
    fn drop(&mut self) {
        if self.tid > 0 {
            #[cfg(debug_assertions)] {
                println!("Closing type: {}", self.tid);
            }
            unsafe { H5Tclose(self.tid) };
        }
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5Type")?;
        writeln!(f, "  type: {}", match self.dtype {
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
        })?;
        writeln!(f, "  tid: {}", self.tid)?;
        writeln!(f, "  size: {}", self.size())?;
        Ok(())
    }
}

pub trait DataTypeOwner {
    fn get_type(&self) -> Result<DataType, Error>;
}
