use crate::phaseh5::PyPhaseH5;
use pyo3::prelude::*;
use spike_rs::{analisys, operations};
use std::collections::HashMap;

#[pyfunction]
pub fn spike_detection(
    data: Vec<f32>,
    sampling_frequency: f32,
    threshold: f32,
    peak_duration: f32,
    refractory_time: f32,
) -> Option<(Vec<usize>, Vec<f32>)> {
    match operations::spike_detection(
        &data[..],
        sampling_frequency,
        threshold,
        peak_duration,
        refractory_time,
    ) {
        Ok(ret) => Some(ret),
        Err(err) => {
            println!("Pycode_rs::compute_threshold: Error {err}");
            None
        }
    }
}

#[pyfunction]
pub fn compute_threshold(
    data: Vec<f32>,
    sampling_frequency: f32,
    multiplier: f32,
) -> Option<f32> {
    match operations::compute_threshold(&data, sampling_frequency, multiplier) {
        Ok(ret) => Some(ret),
        Err(err) => {
            println!("Pycode_rs::compute_threshold: Error {err}");
            None
        }
    }
}

#[pyfunction]
pub fn subsample_peak_trains(
    phase: &mut PyPhaseH5,
    bin_size: usize,
    digital_index: usize,
) -> Option<HashMap<String, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>>> {
    let subsample_ret = analisys::subsample_peak_trains(
        phase._phaseh5.as_mut().unwrap(),
        bin_size,
        digital_index,
    );
    match subsample_ret {
        Ok(res) => Some(res),
        Err(err) => {
            eprintln!("Pycode_rs::Error: psth {err}");
            None
        }
    }
}

#[pyfunction]
pub fn subsampled_post_stimulus_times(
    phase: &mut PyPhaseH5,
    bin_size: usize,
    n_bins_post_stim: usize,
    digital_index: usize,
) -> Option<Vec<Vec<usize>>> {
    let psth_res = analisys::subsampled_post_stimulus_times(
        phase._phaseh5.as_mut().unwrap(),
        bin_size,
        n_bins_post_stim,
        digital_index,
    );
    match psth_res {
        Ok(res) => Some(res),
        Err(err) => {
            eprintln!("Pycode_rs::Error: psth {err}");
            None
        }
    }
}

#[pyfunction]
pub fn get_digital_intervals(digital: Vec<f32>) -> Vec<(usize, usize)> {
    operations::get_digital_intervals(&digital)
}

#[pyfunction]
pub fn subsample_range(
    peaks: Vec<usize>,
    starting_sample: usize,
    bin_size: usize,
    n_bins: usize,
) -> Vec<usize> {
    operations::subsample_range(&peaks, starting_sample, bin_size, n_bins)
}
