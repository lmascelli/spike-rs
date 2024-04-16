////////////////////////////////////////////////////////////////////////////////
use ::spike_rs::core::{operations, types::Phase};
use pyo3::prelude::*;
///                                 PyPhase Class
use std::collections::HashMap;

#[pyclass(name = "Phase")]
pub struct PyPhase {
    #[pyo3(get)]
    sampling_frequency: f32,

    #[pyo3(get)]
    channel_labels: Vec<String>,
    #[pyo3(get)]
    raw_data_lengths: HashMap<String, usize>,
    #[pyo3(get)]
    peak_train_lengths: HashMap<String, usize>,

    #[pyo3(get)]
    digitals_lengths: Vec<usize>,

    phase: Phase,
}

impl PyPhase {
    pub fn from(phase: Phase) -> Self {
        let mut ret = PyPhase::new();
        ret.sampling_frequency = phase.sampling_frequency;
        ret.phase = phase;
        ret.update();
        ret
    }
}

#[pymethods]
impl PyPhase {
    #[new]
    pub fn new() -> Self {
        PyPhase {
            sampling_frequency: 0f32,
            channel_labels: vec![],
            raw_data_lengths: HashMap::new(),
            peak_train_lengths: HashMap::new(),
            digitals_lengths: vec![],
            phase: Phase::default(),
        }
    }

    fn update(&mut self) {
        let mut raw_data_lengths = HashMap::new();
        let mut peak_train_lengths = HashMap::new();

        for (label, data) in &self.phase.raw_data {
            raw_data_lengths.insert(label.clone(), data.len());
        }

        for (label, data) in &self.phase.peaks_trains {
            peak_train_lengths.insert(label.clone(), data.0.len());
        }

        self.channel_labels = self.phase.raw_data.keys().map(|x| x.clone()).collect();
        self.raw_data_lengths = raw_data_lengths;
        self.peak_train_lengths = peak_train_lengths;
        self.digitals_lengths = self.phase.digitals.iter().map(|x| x.len()).collect();
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

    pub fn get_el_stim_intervals(&self) -> Option<Vec<Vec<u64>>> {
        if self.phase.el_stim_intervals.len() == 0 {
            None
        } else {
            Some(self.phase.el_stim_intervals.clone())
        }
    }

    pub fn get_peaks_train(&self, label: &str) -> Option<(Vec<f32>, Vec<usize>)> {
        if let Some(data) = self.phase.peaks_trains.get(label) {
            Some(data.clone())
        } else {
            None
        }
    }

    pub fn compute_all_peak_trains(
        &mut self,
        peak_duration: f32,
        refractary_time: f32,
        n_devs: f32,
    ) {
        self.phase
            .compute_all_peak_trains(peak_duration, refractary_time, n_devs);
        self.update();
    }

    // pub fn get_peaks_stats(&self) -> Vec<(f32, f32)> {
    //     let mut ret = vec![];
    //     for (_, (peaks_values, _)) in &self.phase.peaks_trains {
    //         ret.push((operations::math::mean(&peaks_values[..]),
    //             *peaks_values.iter().max_by(|x, y|
    //                 x.abs().partial_cmp(&y.abs()).unwrap())
    //                 .unwrap()));
    //     }
    //     ret
    // }

    pub fn clear_peaks_over_threshold(&mut self, threshold: f32) {
        self.phase.clear_peaks_over_threshold(threshold);

        let mut peak_train_lengths = HashMap::new();
        for (label, data) in &self.phase.peaks_trains {
            peak_train_lengths.insert(label.clone(), data.0.len());
        }
        self.peak_train_lengths = peak_train_lengths;
    }

    pub fn get_peaks_bins(&self, n_bins: usize) -> HashMap<String, (Vec<usize>, f32, f32)> {
        let mut ret = HashMap::new();

        for (label, (peaks_values, _peaks_times)) in &self.phase.peaks_trains {
            ret.insert(
                label.clone(),
                operations::get_peaks_bins(&peaks_values[..], n_bins).unwrap_or((
                    Vec::new(),
                    0f32,
                    0f32,
                )),
            );
        }

        ret
    }

    // pub fn spikes_count(&self, label: &str) -> usize {
    //     return self.phase.peaks_trains[label].0.len();
    // }

    pub fn get_digital_intervals(&self, index: usize) -> Option<Vec<(usize, usize)>> {
        if index >= self.digitals_lengths.len() {
            None
        } else {
            Some(operations::get_digital_intervals(
                &self.phase.digitals[index][..],
            ))
        }
    }

    pub fn get_peaks_in_consecutive_intervals(
        &self,
        intervals: Vec<(usize, usize)>,
    ) -> HashMap<String, (Vec<f32>, Vec<usize>)> {
        self.phase.get_peaks_in_consecutive_intervals(&intervals)
    }

    pub fn get_peaks_in_interval(
        &self,
        interval: (usize, usize),
    ) -> HashMap<String, (Vec<f32>, Vec<usize>)> {
        self.phase.get_peaks_in_interval(&interval)
    }

    pub fn get_subsampled_pre_stim_post_from_intervals(
        &self,
        intervals: Vec<(usize, usize)>,
        bin_size: usize,
    ) -> HashMap<String, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>> {
        self.phase
            .get_subsampled_pre_stim_post_from_intervals(&intervals, bin_size)
    }

    pub fn psth(&self, bin_size: usize, digital_index: usize) -> Option<Vec<Vec<usize>>> {
        match self.phase.psth(bin_size, digital_index) {
            Ok(ret) => Some(ret),
            Err(_err) => None,
        }
    }
}
////////////////////////////////////////////////////////////////////////////////
///                                 Global functions

#[pyfunction]
pub fn load_phase(filename: &str) -> Option<PyPhase> {
    if let Ok(phase) = mc_explorer::old::load_phase(filename) {
        Some(PyPhase::from(phase))
    } else {
        None
    }
}

#[pyfunction]
pub fn save_phase(phase: &PyPhase, filename: &str) -> bool {
    if let Ok(_) = mc_explorer::old::save_phase(&phase.phase, filename) {
        true
    } else {
        false
    }
}

// #[pyfunction]
// pub fn convert_mc_h5_file(source: &str, dest: &str) -> usize {
//     let phase_r;
//     {
//         phase_r = hdf5::converter::convert_mc_h5_file(source);
//     }
//     if let Ok(phase) = phase_r {
//         if let Ok(_) = hdf5::io::save_phase(&phase, dest) {
//             return 0usize;
//         } else {
//             return 1usize;
//         }
//     } else {
//         return 1usize;
//     }
// }

#[pyfunction]
pub fn check_valid_bin_size(interval: (usize, usize), bin_size: usize) -> usize {
    operations::check_valid_bin_size(interval, bin_size)
}
