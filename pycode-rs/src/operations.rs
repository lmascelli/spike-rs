use pyo3::prelude::*;
use spike_rs::operations::{self};

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
        Err(err) => None,
    }
}

#[pyfunction]
pub fn compute_threshold(
    data: Vec<f32>,
    sampling_frequency: f32,
    multiplier: f32,
) -> Option<f32> {
    match operations::compute_threshold(
        &data,
        sampling_frequency,
        multiplier,
    ) {
        Ok(ret) => Some(ret),
        Err(err) => None,
    }
}
