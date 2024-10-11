////////////////////////////////////////////////////////////////////////////////
///
///                             Python Wrapper
///
////////////////////////////////////////////////////////////////////////////////

use pyo3::prelude::*;
pub mod phaseh5;
pub mod operations;

#[pymodule]
fn pycode_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<phaseh5::PyPhaseH5>()?;
    m.add_wrapped(wrap_pyfunction!(operations::spike_detection))?;
    m.add_wrapped(wrap_pyfunction!(operations::compute_threshold))?;
    m.add_wrapped(wrap_pyfunction!(operations::get_digital_intervals))?;
    m.add_wrapped(wrap_pyfunction!(operations::subsample_peak_trains))?;
    m.add_wrapped(wrap_pyfunction!(operations::subsampled_post_stimulus_times))?;
    m.add_wrapped(wrap_pyfunction!(operations::subsample_range))?;
    Ok(())
}
