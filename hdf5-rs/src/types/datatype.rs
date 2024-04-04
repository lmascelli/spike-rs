use crate::h5sys::*;

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
    pub fn parse(dtype_id: i64) -> Self {
        unsafe {
            match H5Tget_class(dtype_id) {
                H5T_class_t_H5T_INTEGER => {
                    let sign = match H5Tget_sign(dtype_id) {
                        H5T_sign_t_H5T_SGN_2 => true,
                        H5T_sign_t_H5T_SGN_NONE => false,
                        _ => {
                            return DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Unimplemented,
                            };
                        }
                    };
                    let size = H5Tget_size(dtype_id);
                    if sign {
                        if size == 4 {
                            return DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Signed32,
                            };
                        }
                        else if size == 8 {
                            return DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Signed64,
                            };
                        } else {
                            return DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Unimplemented,
                            };
                        }
                    }
                    if size == 4 {
                        DataType {
                            tid: dtype_id,
                            dtype: DataTypeL::Unsigned32,
                        }
                    }
                    else if size == 8 {
                        DataType {
                            tid: dtype_id,
                            dtype: DataTypeL::Unsigned64,
                        }
                    } else {
                        DataType {
                            tid: dtype_id,
                            dtype: DataTypeL::Unimplemented,
                        }
                    }
                },

                H5T_class_t_H5T_FLOAT => {
                    match H5Tget_size(dtype_id) {
                        4 => {
                            DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Float32,
                            }
                        },
                        8 => {
                            DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Float64,
                            }
                        },
                        _ => {
                            DataType {
                                tid: dtype_id,
                                dtype: DataTypeL::Unimplemented,
                            }
                        },
                    }
                },

                H5T_class_t_H5T_STRING => {
                    let padding = H5Tget_strpad(dtype_id);
                    let cset = H5Tget_cset(dtype_id);
                    let is_variable = {
                        let r = H5Tis_variable_str(dtype_id);
                        match r {
                            0   => true,
                            _ if r > 0 => false,
                            _   => {
                                panic!("DataType::parse: Failed to retrieve if the string type is variable");
                            },
                        }
                        // if r > 0 {
                        //     true
                        // } else if r == 0 {
                        //     false
                        // } else {
                        // }
                    };

                    if padding == H5T_str_t_H5T_STR_NULLTERM && cset == H5T_cset_t_H5T_CSET_ASCII {
                        DataType {
                            tid: dtype_id,
                            dtype: if is_variable {
                                DataTypeL::StringDynamic
                            } else {
                                DataTypeL::StringStatic
                            },
                        }
                    } else {
                        panic!("DataType::parse: only ascii c null terminated string are supported");
                    }
                },

                H5T_class_t_H5T_COMPOUND => {
                    todo!()
                },
                _ => {
                    DataType {
                        tid: dtype_id,
                        dtype: DataTypeL::Unimplemented,
                    }
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
            println!("Closing type: {}", self.tid);
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
            DataTypeL::Unimplemented => "Unimplemented type conversion",
        })?;
        writeln!(f, "  tid: {}", self.tid)?;
        writeln!(f, "  size: {}", self.size())?;
        Ok(())
    }
}

pub trait DataTypeOwner {
    fn get_type(&self) -> Result<DataType, String>;
}
