#[derive(Debug)]
pub enum ErrorType {
    AttributeOpenFail(String),
    AttributeGetTypeFail(String),
    AttributeFillFail(String, String),
    AttributeFillNotAvailable(String),

    DataSpaceSimpleNew(Vec<u64>),
    DataSpaceGetDimensionsFail,
    DataSpaceSelectSlabFail(Vec<u64>, Vec<u64>, Vec<u64>),
    DataSpaceSelectSlabOutOfBounds(Vec<u64>, Vec<u64>, Vec<u64>),
    DataSpaceSelectRowNotBidimensional(Vec<u64>),

    DataTypeGetTypeFail,
    DataTypeParseStringIsVariableFail,
    DataTypeParseStringTypeNotSupported,

    DataSetOpenFail(String),
    DataSetHasNoDataSpace(String),
    DataSetHasNoDataType(String),
    DataSetUnvalidType(String, String),
    DataSetFillMemoryFail(String),

    FileCreate(String),
    FileOpen(String),

    GroupDoesntExist(String),
    GroupOpenFail(String),

    PListCreate,
    // PListCopy,
    
    NotYetImplemented(Option<String>),
    OtherWithString(String),
    Other,
}

#[derive(Debug)]
pub struct Error {
    etype: ErrorType,
}

impl Error {
    pub fn other() -> Self {
        Self {
            etype: ErrorType::Other,
        }
    }

    pub fn other_with_string(s: &str) -> Self {
        Self {
            etype: ErrorType::OtherWithString(s.to_string())
        }
    }

    pub fn not_yet_implemented(data: Option<&str>) -> Self {
        Self {
            etype: ErrorType::NotYetImplemented(
                       if let Some(data) = data {
                           Some(data.to_string())
                       } else {None}),
        }
    }

    // FILE ERRORS
    
    pub fn file_create(filename: &str) -> Self {
        Self {
            etype: ErrorType::FileCreate(filename.to_string()),
        }
    }
    
    pub fn file_open(filename: &str) -> Self {
        Self {
            etype: ErrorType::FileOpen(filename.to_string()),
        }
    }

    // GROUP ERRORS
    pub fn group_open(name: &str) -> Self {
        Self {
            etype: ErrorType::GroupOpenFail(name.to_string()),
        }
    }

    pub fn group_doesnt_exists(name: &str) -> Self {
        Self {
            etype: ErrorType::GroupDoesntExist(name.to_string()),
        }
    }

    // DATASPACE ERRORS

    pub fn dataspace_select_slab_fail(start: &[u64], offset: &[u64], dims: &[u64]) -> Self {
        Self {
            etype: ErrorType::DataSpaceSelectSlabFail(
                       start.iter().map(|x| *x).collect(),
                       offset.iter().map(|x| *x).collect(),
                       dims.iter().map(|x| *x).collect(),
                       ),
        }
    }
    
    pub fn dataspace_select_slab_out_of_boulds(start: &[u64], offset: &[u64], dims: &[u64]) -> Self {
        Self {
            etype: ErrorType::DataSpaceSelectSlabOutOfBounds(
                       start.iter().map(|x| *x).collect(),
                       offset.iter().map(|x| *x).collect(),
                       dims.iter().map(|x| *x).collect(),
                       ),
        }
    }
    
    pub fn dataspace_select_row_not_bidimensional(dims: &[u64]) -> Self {
        Self {
            etype: ErrorType::DataSpaceSelectRowNotBidimensional(dims.iter().map(|x| *x).collect()),
        }
    }
    
    pub fn dataspace_simple_new(dim: &[u64]) -> Self {
        Self {
            etype: ErrorType::DataSpaceSimpleNew(dim.iter().map(|x| *x).collect()),
        }
    }

    pub fn dataspace_get_dimensions_fail() -> Self {
        Self {
            etype: ErrorType::DataSpaceGetDimensionsFail,
        }
    }

    pub fn dataset_fill_memory_fail(path: &str) -> Self {
        Self {
            etype: ErrorType::DataSetFillMemoryFail(path.to_string()),
        }
    }

    // DATATYPE ERRORS

    pub fn datatype_get_type_fail() -> Self {
        Self {
            etype: ErrorType::DataTypeGetTypeFail,
        }
    }

    pub fn datatype_parse_string_is_variable() -> Self {
        Self {
            etype: ErrorType::DataTypeParseStringIsVariableFail,
        }
    }

    pub fn datatype_parse_string_type_not_supported() -> Self {
        Self {
            etype: ErrorType::DataTypeParseStringTypeNotSupported,
        }
    }

    // DATASET ERRORS
    
    pub fn dataset_open_fail(path: &str) -> Self {
        Self {
            etype: ErrorType::DataSetOpenFail(path.to_string()),
        }
    }

    pub fn dataset_has_no_dataspace(path: &str) -> Self {
        Self {
            etype: ErrorType::DataSetHasNoDataSpace(path.to_string()),
        }
    }

    pub fn dataset_has_no_datatype(path: &str) -> Self {
        Self {
            etype: ErrorType::DataSetHasNoDataType(path.to_string()),
        }
    }

    pub fn dataset_unvalid_type(path: &str, typename: &str) -> Self {
        Self {
            etype: ErrorType::DataSetUnvalidType(path.to_string(), typename.to_string()),
        }
    }

    // ATTRIBUTE ERRORS
    
    pub fn attribute_open(name: &str) -> Self {
        Self {
            etype: ErrorType::AttributeOpenFail(name.to_string()),
        }
    }

    pub fn attribute_get_type(name: &str) -> Self {
        Self {
            etype: ErrorType::AttributeGetTypeFail(name.to_string()),
        }
    }

    pub fn attribute_fill_fail(from: &str, to: &str) -> Self {
        Self {
            etype: ErrorType::AttributeFillFail(from.to_string(), to.to_string()),
        }
    }

    pub fn attribute_fill_not_available(to: &str) -> Self {
        Self {
            etype: ErrorType::AttributeFillNotAvailable(to.to_string()),
        }
    }

    // PLIST ERRORS
    pub fn plist_create() -> Self {
        Self {
            etype: ErrorType::PListCreate,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &'_ mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self.etype {
            ErrorType::FileCreate(ref filename) => {
                writeln!(f, "Error::FileCreate: failed to create file {}", filename)?;
            },

            ErrorType::FileOpen(ref filename) => {
                writeln!(f, "Error::FileOpen: failed to open file {}", filename)?;
            },

            ErrorType::GroupOpenFail(ref name) => {
                writeln!(f, "Error::GroupOpenFail: failed to open group {}", name)?;
            },

            ErrorType::GroupDoesntExist(ref name) => {
                writeln!(f, "Error::GroupDoesntExist: {}", name)?;
            },

            ErrorType::DataSpaceGetDimensionsFail => {
                writeln!(f, "Error::DataSpaceGetDimensionsFail")?;
            },

            ErrorType::DataSpaceSimpleNew(ref dims) => {
                writeln!(f, "Error::DataSpaceSimpleNew: dims: {:?}", dims)?;
            },

            ErrorType::DataSpaceSelectSlabFail(ref start, ref offset, ref dims) => {
                writeln!(f,
                         r#"Error::DataSpaceSelectSlabFail: invalid selection from {:?} with offset {:?}
                         have different rank than dataspace with dimension {:?}"#,
                         start, offset, dims)?;

            },

            ErrorType::DataSpaceSelectSlabOutOfBounds(ref start, ref offset, ref dims) => {
                writeln!(f,
                         r#"Error::DataSpaceSelectSlabOutOfBounds: invalid selection from {:?} with offset {:?}
                         have different rank than dataspace with dimension {:?}"#,
                         start, offset, dims)?;

            },

            ErrorType::DataSpaceSelectRowNotBidimensional(ref dims) => {
                writeln!(f, r#"Error::DataSpaceSelectRowNotBidimensional: select_row: select_row is valid 
                            only for bidimensional dataspaces. Current dataspace dimensions: {:?}"#, dims)?;
            },

            ErrorType::DataTypeGetTypeFail => {
                writeln!(f, "Error:DataTypeGetTypeFail: opening datatype returned and unvalid id")?;
            },

            ErrorType::DataTypeParseStringIsVariableFail => {
                writeln!(f, r#"Error:DataTypeParseStringIsVariableFail: failed to retrieve if the string
                            type is variable"#)?;
            },

            ErrorType::DataTypeParseStringTypeNotSupported => {
                writeln!(f, r#"Error:DataTypeParseStringTypeNotSupported: only ascii c null terminated 
                            string are supported"#)?;
            },

            ErrorType::DataSetOpenFail(ref path) => {
                writeln!(f, "Error::DataSetOpenFail: {}", path)?;
            },

            ErrorType::DataSetHasNoDataSpace(ref path) => {
                writeln!(f, "Error::DataSetHasNoDataSpace: {}", path)?;
            },

            ErrorType::DataSetHasNoDataType(ref path) => {
                writeln!(f, "Error::DataSetHasNoDataType: {}", path)?;
            },

            ErrorType::DataSetUnvalidType(ref path, ref typename) => {
                writeln!(f, "Error::DataSetUnvalidType: cannot read type {}, from dataset {}",
                         typename, path)?;
            }

            ErrorType::DataSetFillMemoryFail(ref path) => {
                writeln!(f, "Error::DataSetFillMemoryFail: failed to fill memory from dataset {}", path)?;
            },

            ErrorType::AttributeOpenFail(ref name) => {
                writeln!(f, "Error::AttributeOpenFail: {}", name)?;
            },

            ErrorType::AttributeGetTypeFail(ref name) => {
                writeln!(f, "Error::AttributeGetTypeFail: {}", name)?;
            },
            
            ErrorType::AttributeFillFail(ref from, ref to) => {
                writeln!(f, "Error::AttributeFillFail: from {} to {}", from, to)?;
            },

            ErrorType::AttributeFillNotAvailable(ref to) => {
                writeln!(f, "Error::AttributeFillNotAvailable: cannot fill the attribute into {}", to)?;
            },

            ErrorType::PListCreate => {
                writeln!(f, "Error::PListCreate: failed to create the property list")?;
            },

            ErrorType::NotYetImplemented(ref data) => {
                writeln!(f, "This feature has not yet been implemented{}", 
                         if let Some(data) = data {format!(": {}", data )} else {"".to_string()})?;
            },

            ErrorType::OtherWithString(ref data) => {
                writeln!(f, "Error::OtherWithString: {}", data)?;
            },

            _ => {
                writeln!(f, "Error::Other: unknown error")?;
            }
        }
        Ok(())
    }
}
