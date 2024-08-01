////////////////////////////////////////////////////////////////////////////////
///
///                             Python Wrapper
///
////////////////////////////////////////////////////////////////////////////////

use pyo3::prelude::*;
mod phaseh5;
mod operations;

#[pymodule]
fn pycode_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<phaseh5::PyPhaseH5>()?;
    m.add_wrapped(wrap_pyfunction!(operations::spike_detection))?;
    m.add_wrapped(wrap_pyfunction!(operations::compute_threshold))?;
    Ok(())
}
