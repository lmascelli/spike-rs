#[derive(Debug)]
pub enum ErrorType {
    AttributeOpenFail(String),
    AttributeGetTypeFail(String),
    // AttributeGetDataTypeFail,
    // AttributeGetDataSpaceFail,
    // AttributeFillFail,
    // AttributeFillNotAvailable,
    DataSetHasNoDataSpace(String),
    DataSetHasNoDataType(String),
    // DataSetUnvalidType,
    // DataSpaceOpenFail,
    // DataSpaceGetDimensionsFail,
    // DataSpaceGetSpaceFail,
    // DataSpaceSelectSlabOutOfBounds,
    // DataSpaceSelectSlabFail,
    // DataSpaceSelectRowNotBidimensional,
    // DataTypeGetTypeFail,
    FileOpen(String),
    GroupDoesntExist(String),
    GroupOpenFail(String),
    PListCreate,
    // PListCopy,
    NotYetImplemented(Option(String)),
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

    pub fn not_yet_implemented(data: Option<&str>) -> Self {
        Self {
            etype: ErrorType::NotYetImplemented(
                       if Some(data) = data {
                           data.to_string()
                       } else {None}),
        }
    }

    // FILE ERRORS
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

    // DATASET ERRORS
    
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
            ErrorType::FileOpen(filename) => {
                writeln!(f, "Error::FileOpen: failed to open file {}", filename)?;
            },

            ErrorType::GroupOpenFail(name) => {
                writeln!(f, "Error::GroupOpenFail: failed to open group {}", name)?;
            },

            ErrorType::GroupDoesntExist(name) => {
                writeln!(f, "Error::GroupDoesntExist: {}", name)?;
            },

            ErrorType::DataSetHasNoDataSpace(path) => {
                writeln!(f, "Error::DataSetHasNoDataSpace: {}", path)?;
            },

            ErrorType::DataSetHasNoDataType(path) => {
                writeln!(f, "Error::DataSetHasNoDataType: {}", path)?;
            },

            ErrorType::AttributeOpenFail(name) => {
                writeln!(f, "Error::AttributeOpenFail: {}", name)?;
            },

            ErrorType::AttributeGetTypeFail(name) => {
                writeln!(f, "Error::AttributeGetTypeFail: {}", name)?;
            },

            ErrorType::PListCreate => {
                writeln!(f, "Error::PListCreate: failed to create the property list")?;
            },

            ErrorType::NotYetImplemented(data) => {
                writeln!(f, "This feature has not yet been implemented{}", 
                         if Some(data) = data {format!(": {}", data )} else {""})?;
            },

            _ => {
                writeln!(f, "Error::Other: unknown error")?;
            }
        }
        Ok(())
    }
}
