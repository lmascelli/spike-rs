////////////////////////////////////////////////////////////////////////////////
///
///                             Python Wrapper
///
////////////////////////////////////////////////////////////////////////////////

use pyo3::prelude::*;
mod phaseh5;

// mod pyphase;
// mod mc_explorer;
// use mc_explorer::PyMCExplorer;
// use pyphase::{load_phase, save_phase};
// use pyphase::PyPhase;

#[pymodule]
fn pycode_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<phaseh5::PyPhaseH5>()?;
    // m.add_function(wrap_pyfunction!(load_phase, m)?)?;
    // m.add_function(wrap_pyfunction!(save_phase, m)?)?;
    // m.add_class::<PyPhase>()?;
    // m.add_class::<PyMCExplorer>()?;
    Ok(())
}
