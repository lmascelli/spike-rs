pub mod core;
pub mod hdf5;

////////////////////////////////////////////////////////////////////////////////
///
///                             Python Wrapper
///
////////////////////////////////////////////////////////////////////////////////
use pyo3::prelude::*;

#[pyclass(name = "Phase")]
struct PyPhase {
    #[pyo3(get)]
    channel_labels: Vec<String>,
    #[pyo3(get)]
    digitals_num: usize,
    phase: core::types::Phase,
}

impl PyPhase {
    fn from(phase: core::types::Phase) -> Self {
        PyPhase {
            channel_labels: phase.raw_data.keys().map(|x| x.clone()).collect(),
            digitals_num: phase.digitals.len(),
            phase: phase,
        }
    }
}

#[pymethods]
impl PyPhase {
    #[new]
    pub fn new() -> Self {
        PyPhase {
            phase: core::types::Phase::default(),
            digitals_num: 0,
            channel_labels: vec![],
        }
    }

    pub fn get_digital(&self, index: usize) -> Option<Vec<f32>> {
        if index >= self.phase.digitals.len() {
            None
        } else {
            Some(self.phase.digitals[index].clone())
        }
    }

    pub fn get_raw_data(&self, label: &str) -> Option<Vec<f32>> {
        if let Some(data) = self.phase.raw_data.get(label) {
            Some(data.clone())
        } else {
            None
        }
    }

    pub fn get_peaks_train(&self, label: &str) -> Option<(Vec<f32>, Vec<usize>)> {
        if let Some(data) = self.phase.peaks_trains.get(label) {
            Some(data.clone())
        } else {
            None
        }
    }

}

#[pyfunction]
fn load_phase(filename: &str) -> Option<PyPhase> {
    if let Ok(phase) = hdf5::load_phase(filename) {
        Some(PyPhase::from(phase))
    } else {
        None
    }
}

#[pymodule]
fn spyke_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_phase, m)?)?;
    m.add_class::<PyPhase>()?;
    Ok(())
}
