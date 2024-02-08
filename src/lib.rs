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
    sampling_frequency: f32,

    #[pyo3(get)]
    channel_labels: Vec<String>,
    #[pyo3(get)]
    raw_data_lengths: Vec<usize>,
    #[pyo3(get)]
    peak_train_lengths: Vec<usize>,

    #[pyo3(get)]
    digitals_lengths: Vec<usize>,

    phase: core::types::Phase,
}

impl PyPhase {

    fn from(phase: core::types::Phase) -> Self {
        PyPhase {
            sampling_frequency: phase.sampling_frequency,
            channel_labels: phase.raw_data.keys().map(|x| x.clone()).collect(),
            raw_data_lengths: phase.raw_data.keys().map(|x| phase.raw_data[x].len()).collect(),
            peak_train_lengths: phase.peaks_trains.keys().map(|x| phase.peaks_trains[x].0.len()).collect(),
            digitals_lengths: phase.digitals.iter().map(|x| x.len()).collect(),
            phase: phase,
        }
    }
}

#[pymethods]
impl PyPhase {

    #[new]
    pub fn new() -> Self {
        PyPhase {
            sampling_frequency: 0f32,
            channel_labels: vec![],
            raw_data_lengths: vec![],
            peak_train_lengths: vec![],
            digitals_lengths: vec![],
            phase: core::types::Phase::default(),
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

    pub fn get_peaks_train(&self, label: &str) -> Option<(Vec<f32>,
                                                          Vec<usize>)> {
        if let Some(data) = self.phase.peaks_trains.get(label) {
            Some(data.clone())
        } else {
            None
        }
    }

    pub fn compute_all_peak_trains(&mut self, peak_duration: f32,
                                   refractary_time: f32, n_devs: f32) {
        self.phase.compute_all_peak_trains(peak_duration, refractary_time,
                                     n_devs);
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
