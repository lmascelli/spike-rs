use pyo3::prelude::*;
use mc_explorer::H5Content;

#[pyclass(name = "MCExplorer")]
pub struct PyMCExplorer {
    content: Option<H5Content>,
}

#[pymethods]
impl PyMCExplorer {
    #[new]
    pub fn new(filename: &str) -> Self {
        let content = H5Content::open(filename);
        Self {
            content: if let Ok(content) = content{
                println!("{} file loaded", filename);
                Some(content)
            } else {
                println!("PyMCExplorer: {}", content.err().unwrap());
                None
            }
        }
    }
}
