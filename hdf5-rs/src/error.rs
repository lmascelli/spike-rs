#[derive(Debug)]
pub enum H5ErrorType {
    AttributeOpenFail(String),
    AttributeGetTypeFail(String),
    AttributeFillFail(String, String),
    AttributeFillNotAvailable(String),

    DataSpaceSimpleNew(Vec<u64>),
    DataSpaceGetDimensionsFail,
    DataSpaceSelectSlabFail(Vec<u64>, Vec<u64>, Vec<u64>),
    DataSpaceSelectSlabOutOfBounds(Vec<u64>, Vec<u64>, Vec<u64>),
    DataSpaceNotBidimensional(Vec<u64>),
    DataSpaceOutOfBounds(Vec<u64>),

    DataTypeCopyFail,
    DataTypeGetTypeFail,
    DataTypeParseStringIsVariableFail,
    DataTypeParseStringTypeNotSupported,

    DataSetCreationFailed(String),
    DataSetAlreadyExists(String),
    DataSetOpenFail(String),
    DataSetHasNoDataSpace(String),
    DataSetHasNoDataType(String),
    DataSetUnvalidType(String, String),
    DataSetFillMemoryFail(String),
    DataSetFromSubspaceStartPointWrongDims,
    DataSetFromSubspaceStartAndOffsetHaveDifferentDims,
    DataSetWriteFail(String, String),
    DataSetWriteToDataSetDifferentLength(u64, u64),

    FileCreate(String),
    FileOpen(String),

    FilterRegistrationFailed(String),

    GroupDoesntExist(String),
    GroupOpenFail(String),
    GroupCreationFailed(String),
    GroupAlreadyExists(String),

    IdUnvalid,

    LibraryInitFail,

    LinkGetTypeFailed,

    PListCreate,
    PListClassesDoNotMatch(String, String),
    PListSetFilterWrongNumberOfParameter,
    PListGetChunkFail,
    PListNotDatasetAccess,

    NotYetImplemented(Option<String>),
    OtherWithString(String),
    Other,
}

#[derive(Debug)]
pub struct H5Error {
    // eid: crate::h5sys::types::Hid,
    etype: H5ErrorType,
}

impl H5Error {
    pub fn other() -> Self {
        Self { etype: H5ErrorType::Other }
    }

    pub fn other_with_string(s: &str) -> Self {
        Self { etype: H5ErrorType::OtherWithString(s.to_string()) }
    }

    pub fn not_yet_implemented(data: Option<&str>) -> Self {
        Self {
            etype: H5ErrorType::NotYetImplemented(if let Some(data) = data {
                Some(data.to_string())
            } else {
                None
            }),
        }
    }

    // LIBRARY GENERAL

    pub fn library_init_fail() -> Self {
        Self { etype: H5ErrorType::LibraryInitFail }
    }

    // ID

    pub fn id_unvalid() -> Self {
        Self { etype: H5ErrorType::IdUnvalid }
    }

    // LINK

    pub fn link_get_type_failed() -> Self {
        Self { etype: H5ErrorType::LinkGetTypeFailed }
    }

    // FILE ERRORS

    pub fn file_create(filename: &str) -> Self {
        Self { etype: H5ErrorType::FileCreate(filename.to_string()) }
    }

    pub fn file_open(filename: &str) -> Self {
        Self { etype: H5ErrorType::FileOpen(filename.to_string()) }
    }

    // GROUP ERRORS
    pub fn group_open(name: &str) -> Self {
        Self { etype: H5ErrorType::GroupOpenFail(name.to_string()) }
    }

    pub fn group_doesnt_exists(name: &str) -> Self {
        Self { etype: H5ErrorType::GroupDoesntExist(name.to_string()) }
    }

    pub fn group_creation_failed(name: &str) -> Self {
        Self { etype: H5ErrorType::GroupCreationFailed(name.to_string()) }
    }

    pub fn group_already_exists(name: &str) -> Self {
        Self { etype: H5ErrorType::GroupAlreadyExists(name.to_string()) }
    }

    // DATASPACE ERRORS

    pub fn dataspace_select_slab_fail(
        start: &[u64],
        offset: &[u64],
        dims: &[u64],
    ) -> Self {
        Self {
            etype: H5ErrorType::DataSpaceSelectSlabFail(
                start.iter().map(|x| *x).collect(),
                offset.iter().map(|x| *x).collect(),
                dims.iter().map(|x| *x).collect(),
            ),
        }
    }

    pub fn dataspace_select_slab_out_of_boulds(
        start: &[u64],
        offset: &[u64],
        dims: &[u64],
    ) -> Self {
        Self {
            etype: H5ErrorType::DataSpaceSelectSlabOutOfBounds(
                start.iter().map(|x| *x).collect(),
                offset.iter().map(|x| *x).collect(),
                dims.iter().map(|x| *x).collect(),
            ),
        }
    }

    pub fn dataspace_not_bidimensional(dims: &[u64]) -> Self {
        Self {
            etype: H5ErrorType::DataSpaceNotBidimensional(
                dims.iter().map(|x| *x).collect(),
            ),
        }
    }

    pub fn dataspace_simple_new(dim: &[u64]) -> Self {
        Self {
            etype: H5ErrorType::DataSpaceSimpleNew(
                dim.iter().map(|x| *x).collect(),
            ),
        }
    }

    pub fn dataspace_get_dimensions_fail() -> Self {
        Self { etype: H5ErrorType::DataSpaceGetDimensionsFail }
    }

    pub fn dataset_fill_memory_fail(path: &str) -> Self {
        Self { etype: H5ErrorType::DataSetFillMemoryFail(path.to_string()) }
    }

    pub fn dataspace_out_of_bounds(dims: &[u64]) -> Self {
        Self {
            etype: H5ErrorType::DataSpaceOutOfBounds(
                dims.iter().map(|x| *x).collect(),
            ),
        }
    }

    // DATATYPE ERRORS

    pub fn datatype_copy_fail() -> Self {
        Self { etype: H5ErrorType::DataTypeCopyFail }
    }

    pub fn datatype_get_type_fail() -> Self {
        Self { etype: H5ErrorType::DataTypeGetTypeFail }
    }

    pub fn datatype_parse_string_is_variable() -> Self {
        Self { etype: H5ErrorType::DataTypeParseStringIsVariableFail }
    }

    pub fn datatype_parse_string_type_not_supported() -> Self {
        Self { etype: H5ErrorType::DataTypeParseStringTypeNotSupported }
    }

    // DATASET ERRORS

    pub fn dataset_creation_failed(path: &str) -> Self {
        Self { etype: H5ErrorType::DataSetCreationFailed(path.to_string()) }
    }

    pub fn dataset_already_exists(path: &str) -> Self {
        Self { etype: H5ErrorType::DataSetAlreadyExists(path.to_string()) }
    }

    pub fn dataset_open_fail(path: &str) -> Self {
        Self { etype: H5ErrorType::DataSetOpenFail(path.to_string()) }
    }

    pub fn dataset_has_no_dataspace(path: &str) -> Self {
        Self { etype: H5ErrorType::DataSetHasNoDataSpace(path.to_string()) }
    }

    pub fn dataset_has_no_datatype(path: &str) -> Self {
        Self { etype: H5ErrorType::DataSetHasNoDataType(path.to_string()) }
    }

    pub fn dataset_unvalid_type(path: &str, typename: &str) -> Self {
        Self {
            etype: H5ErrorType::DataSetUnvalidType(
                path.to_string(),
                typename.to_string(),
            ),
        }
    }

    pub fn dataset_from_subspace_start_point_wrong_dims() -> Self {
        Self { etype: H5ErrorType::DataSetFromSubspaceStartPointWrongDims }
    }

    pub fn dataset_from_subspace_start_and_offset_have_different_dims() -> Self
    {
        Self {
            etype:
                H5ErrorType::DataSetFromSubspaceStartAndOffsetHaveDifferentDims,
        }
    }

    pub fn dataset_write_fail(path: &str, dtype: &str) -> Self {
        Self {
            etype: H5ErrorType::DataSetWriteFail(
                path.to_string(),
                dtype.to_string(),
            ),
        }
    }

    pub fn dataset_write_to_dataset_different_lenght(
        dataspace_shape: &[u64],
        data_shape: &[u64],
    ) -> Self {
        Self {
            etype: H5ErrorType::DataSetWriteToDataSetDifferentLength(
                dataspace_shape.iter().product(),
                data_shape.iter().product(),
            ),
        }
    }

    // ATTRIBUTE ERRORS

    pub fn attribute_open(name: &str) -> Self {
        Self { etype: H5ErrorType::AttributeOpenFail(name.to_string()) }
    }

    pub fn attribute_get_type(name: &str) -> Self {
        Self { etype: H5ErrorType::AttributeGetTypeFail(name.to_string()) }
    }

    pub fn attribute_fill_fail(from: &str, to: &str) -> Self {
        Self {
            etype: H5ErrorType::AttributeFillFail(
                from.to_string(),
                to.to_string(),
            ),
        }
    }

    pub fn attribute_fill_not_available(to: &str) -> Self {
        Self { etype: H5ErrorType::AttributeFillNotAvailable(to.to_string()) }
    }

    // PLIST ERRORS
    pub fn plist_create() -> Self {
        Self { etype: H5ErrorType::PListCreate }
    }

    pub fn plist_classes_do_not_match(
        expected_class: &str,
        actual_class: &str,
    ) -> Self {
        Self {
            etype: H5ErrorType::PListClassesDoNotMatch(
                expected_class.to_string(),
                actual_class.to_string(),
            ),
        }
    }

    pub fn plist_not_dataset_access() -> Self {
        Self { etype: H5ErrorType::PListNotDatasetAccess }
    }

    pub fn plist_set_filter_wrong_number_of_parameters() -> Self {
        Self { etype: H5ErrorType::PListSetFilterWrongNumberOfParameter }
    }

    pub fn plist_get_chunk_fail() -> Self {
        Self { etype: H5ErrorType::PListGetChunkFail }
    }

    // FILTER ERRORS
    pub fn filter_registration_failed(filter_name: &str) -> Self {
        Self {
            etype: H5ErrorType::FilterRegistrationFailed(
                filter_name.to_string(),
            ),
        }
    }
}

impl std::fmt::Display for H5Error {
    fn fmt(
        &self,
        f: &'_ mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        match self.etype {
            H5ErrorType::LibraryInitFail => {
                writeln!(f, "ErrorType::LibraryInitFail: failed to initialize the library")?;
            }

            H5ErrorType::IdUnvalid => {
                writeln!(f, "ErrorType::IdUnvalid")?;
            }

            H5ErrorType::LinkGetTypeFailed => {
                writeln!(f, "ErrorType::LinkGetTypeFailed")?;
            }

            H5ErrorType::FileCreate(ref filename) => {
                writeln!(
                    f,
                    "Error::FileCreate: failed to create file {}",
                    filename
                )?;
            }

            H5ErrorType::FileOpen(ref filename) => {
                writeln!(
                    f,
                    "Error::FileOpen: failed to open file {}",
                    filename
                )?;
            }

            H5ErrorType::GroupOpenFail(ref name) => {
                writeln!(
                    f,
                    "Error::GroupOpenFail: failed to open group {}",
                    name
                )?;
            }

            H5ErrorType::GroupDoesntExist(ref name) => {
                writeln!(f, "Error::GroupDoesntExist: {}", name)?;
            }

            H5ErrorType::GroupCreationFailed(ref name) => {
                writeln!(f, "Error::GroupCreationFailed: {}", name)?;
            }

            H5ErrorType::GroupAlreadyExists(ref name) => {
                writeln!(f, "Error::GroupAlreadyExists: {}", name)?;
            }

            H5ErrorType::DataSpaceGetDimensionsFail => {
                writeln!(f, "Error::DataSpaceGetDimensionsFail")?;
            }

            H5ErrorType::DataSpaceSimpleNew(ref dims) => {
                writeln!(f, "Error::DataSpaceSimpleNew: dims: {:?}", dims)?;
            }

            H5ErrorType::DataSpaceSelectSlabFail(
                ref start,
                ref offset,
                ref dims,
            ) => {
                writeln!(
                    f,
                    r#"Error::DataSpaceSelectSlabFail: invalid selection from {:?} with offset {:?}
                         have different rank than dataspace with dimension {:?}"#,
                    start, offset, dims
                )?;
            }

            H5ErrorType::DataSpaceSelectSlabOutOfBounds(
                ref start,
                ref offset,
                ref dims,
            ) => {
                writeln!(
                    f,
                    r#"Error::DataSpaceSelectSlabOutOfBounds: invalid selection from {:?} with offset {:?}
                         have different rank than dataspace with dimension {:?}"#,
                    start, offset, dims
                )?;
            }

            H5ErrorType::DataSpaceNotBidimensional(ref dims) => {
                writeln!(
                    f,
                    r#"Error::DataSpaceNotBidimensional: row operations are valid 
                            only for bidimensional dataspaces. Current dataspace dimensions: {:?}"#,
                    dims
                )?;
            }

            H5ErrorType::DataTypeCopyFail => {
                writeln!(f, "Error:DataTypeCopyFail: failed to create a copy of the datatype")?;
            }

            H5ErrorType::DataTypeGetTypeFail => {
                writeln!(f, "Error:DataTypeGetTypeFail: opening datatype returned and unvalid id")?;
            }

            H5ErrorType::DataTypeParseStringIsVariableFail => {
                writeln!(
                    f,
                    r#"Error:DataTypeParseStringIsVariableFail: failed to retrieve if the string
                            type is variable"#
                )?;
            }

            H5ErrorType::DataTypeParseStringTypeNotSupported => {
                writeln!(
                    f,
                    r#"Error:DataTypeParseStringTypeNotSupported: only ascii c null terminated 
                            string are supported"#
                )?;
            }

            H5ErrorType::DataSetCreationFailed(ref path) => {
                writeln!(f, "Error::DataSetCreationFail: {}", path)?;
            }

            H5ErrorType::DataSetAlreadyExists(ref path) => {
                writeln!(f, "Error::DataSetAlreadyExists: {}", path)?;
            }

            H5ErrorType::DataSetOpenFail(ref path) => {
                writeln!(f, "Error::DataSetOpenFail: {}", path)?;
            }

            H5ErrorType::DataSetHasNoDataSpace(ref path) => {
                writeln!(f, "Error::DataSetHasNoDataSpace: {}", path)?;
            }

            H5ErrorType::DataSetHasNoDataType(ref path) => {
                writeln!(f, "Error::DataSetHasNoDataType: {}", path)?;
            }

            H5ErrorType::DataSetUnvalidType(ref path, ref typename) => {
                writeln!(f, "Error::DataSetUnvalidType: cannot read type {}, from dataset {}",
                    typename, path)?;
            }

            H5ErrorType::DataSetFillMemoryFail(ref path) => {
                writeln!(f, "Error::DataSetFillMemoryFail: failed to fill memory from dataset {}", path)?;
            }

            H5ErrorType::DataSetFromSubspaceStartPointWrongDims => {
                writeln!(f, "Error::DataSetFromSubspaceStartPointWrongDims")?;
            }

            H5ErrorType::DataSetFromSubspaceStartAndOffsetHaveDifferentDims => {
                writeln!(
                    f,
                    "Error::DataSetFromSubspaceStartAndOffsetHaveDifferentDims"
                )?;
            }

            H5ErrorType::DataSetWriteToDataSetDifferentLength(
                dataspace_shape,
                data_shape,
            ) => {
                writeln!(
                    f,
                    r#"Error::DataSetWriteToDataSetDifferentShapes: the length of the DataSet {:?}, and the"
                    "length of the data {:?} are different"#,
                    dataspace_shape, data_shape,
                )?;
            }

            H5ErrorType::AttributeOpenFail(ref name) => {
                writeln!(f, "Error::AttributeOpenFail: {}", name)?;
            }

            H5ErrorType::AttributeGetTypeFail(ref name) => {
                writeln!(f, "Error::AttributeGetTypeFail: {}", name)?;
            }

            H5ErrorType::AttributeFillFail(ref from, ref to) => {
                writeln!(
                    f,
                    "Error::AttributeFillFail: from {} to {}",
                    from, to
                )?;
            }

            H5ErrorType::AttributeFillNotAvailable(ref to) => {
                writeln!(f, "Error::AttributeFillNotAvailable: cannot fill the attribute into {}", to)?;
            }

            H5ErrorType::PListCreate => {
                writeln!(
                    f,
                    "Error::PListCreate: failed to create the property list"
                )?;
            }

            H5ErrorType::PListClassesDoNotMatch(
                ref expected_class,
                ref actual_class,
            ) => {
                writeln!(f,
                    "Error::PListClassesDoNotMatch: expected_class: {} --- actual_class: {}",
                    expected_class, actual_class)?;
            }

            H5ErrorType::PListNotDatasetAccess => {
                writeln!(f, "Error::PListNotDatasetAccess")?;
            }

            H5ErrorType::PListSetFilterWrongNumberOfParameter => {
                writeln!(f, "Error::PListSetFilterWrongNumberOfParameter: the number of parameter passed to the filter does not match the filter required parameters")?;
            }

            H5ErrorType::PListGetChunkFail => {
                writeln!(f, "Error::PListGetChunkFail")?;
            }

            H5ErrorType::NotYetImplemented(ref data) => {
                writeln!(
                    f,
                    "This feature has not yet been implemented{}",
                    if let Some(data) = data {
                        format!(": {}", data)
                    } else {
                        "".to_string()
                    }
                )?;
            }

            H5ErrorType::FilterRegistrationFailed(ref filter_name) => {
                writeln!(f,
                    "Error::FilterRegistrationFailed: failed to register the filter {}",
                    filter_name)?;
            }

            H5ErrorType::OtherWithString(ref data) => {
                writeln!(f, "Error::OtherWithString: {}", data)?;
            }

            _ => {
                writeln!(f, "Error::Other: unknown error")?;
            }
        }
        Ok(())
    }
}
