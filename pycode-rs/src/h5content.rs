////////////////////////////////////////////////////////////////////////////////
///                                 H5Content Class


use pyo3::prelude::*;
use ::spike_rs::hdf5::h5explorer::H5Content;

#[pyclass(name = "H5Content")]
pub struct PyH5Content {
    content: Option<H5Content>,
}

#[pymethods]
impl PyH5Content {
    #[new]
    pub fn new(filename: &str) -> Self {
        let content = H5Content::from_file(filename);
        if let Ok(content) = content {
            PyH5Content {
                content: Some(content),
            }
        } else {
            println!("{}", content.err().unwrap());
            PyH5Content {
                content: None,
            }
        }
    }

    pub fn __str__(&self) -> String {
        if let Some(ref content) = self.content {
            format!("{}", content)
        } else {
            "H5Content is empty!".to_string()
        }
    }

    pub fn test(&self) -> Option<()>{
        Some(())
    }
}
