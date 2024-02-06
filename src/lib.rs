use pyo3::prelude::*;
pub mod core;
pub mod hdf5;

#[pyfunction]
fn load_phase(filename: &str) -> PyResult<i64> {
    if let Ok(phase) = hdf5::load_phase(filename) {
        Ok(phase.digitals.len() as i64)
    } else {
        Ok(-1)
    }
}

#[pymodule]
fn pycode_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_phase, m)?)?;
    Ok(())
}
