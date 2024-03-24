////////////////////////////////////////////////////////////////////////////////
///
///                             Python Wrapper
///
////////////////////////////////////////////////////////////////////////////////

mod pyphase;
mod h5content;

use pyo3::prelude::*;
use pyphase::{load_phase, save_phase, convert_mc_h5_file, check_valid_bin_size};
use pyphase::PyPhase;
use h5content::PyH5Content;

#[pymodule]
fn pycode_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_phase, m)?)?;
    m.add_function(wrap_pyfunction!(save_phase, m)?)?;
    m.add_function(wrap_pyfunction!(convert_mc_h5_file, m)?)?;
    m.add_function(wrap_pyfunction!(check_valid_bin_size, m)?)?;
    m.add_class::<PyPhase>()?;
    m.add_class::<PyH5Content>()?;
    Ok(())
}
