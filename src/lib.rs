pub mod core;
pub mod hdf5;

////////////////////////////////////////////////////////////////////////////////
///
///                             Python Wrapper
///
////////////////////////////////////////////////////////////////////////////////
use std::collections::HashMap;
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

    pub fn get_peaks_stats(&self) -> Vec<(f32, f32)> {
        let mut ret = vec![];
        for (_, (peaks_values, _)) in &self.phase.peaks_trains {
            ret.push((core::operations::math::mean(&peaks_values[..]),
                *peaks_values.iter().max_by(|x, y|
                    x.abs().partial_cmp(&y.abs()).unwrap())
                    .unwrap()));
        }
        ret
    }

    pub fn clear_peaks_over_threshold(&mut self, threshold: f32) {
        self.phase.clear_peaks_over_threshold(threshold);
        self.peak_train_lengths = self.phase.peaks_trains
            .keys().map(|x| self.phase.peaks_trains[x].0.len()).collect();
    }

    pub fn get_peaks_bins(&self, n_bins: usize) -> HashMap<String, (Vec<usize>, f32, f32)> {
        let mut ret = HashMap::new();

        for (label, (peaks_values, _peaks_times)) in &self.phase.peaks_trains {
            ret.insert(label.clone(),
                core::operations::get_peaks_bins(&peaks_values[..], n_bins)
                    .unwrap_or((Vec::new(), 0f32, 0f32)));
        }
        
        ret
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

#[pyfunction]
fn save_phase(phase: &PyPhase, filename: &str) -> bool {
    if let Ok(_) = hdf5::save_phase(&phase.phase, filename) {
        true
    } else {
        false
    }
}

#[pymodule]
fn spike_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_phase, m)?)?;
    m.add_function(wrap_pyfunction!(save_phase, m)?)?;
    m.add_class::<PyPhase>()?;
    Ok(())
}
