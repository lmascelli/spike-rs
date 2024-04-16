////////////////////////////////////////////////////////////////////////////////
///
///                             Python Wrapper
///
////////////////////////////////////////////////////////////////////////////////

mod pyphase;
mod mc_explorer;

use pyo3::prelude::*;
use mc_explorer::PyMCExplorer;
use pyphase::{load_phase, save_phase, check_valid_bin_size};
use pyphase::PyPhase;

#[pymodule]
fn pycode_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_phase, m)?)?;
    m.add_function(wrap_pyfunction!(save_phase, m)?)?;
//     m.add_function(wrap_pyfunction!(convert_mc_h5_file, m)?)?;
    m.add_function(wrap_pyfunction!(check_valid_bin_size, m)?)?;
    m.add_class::<PyPhase>()?;
    m.add_class::<PyMCExplorer>()?;
    Ok(())
}
