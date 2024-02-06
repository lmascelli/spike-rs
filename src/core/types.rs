use crate::core::operations::{compute_threshold, spike_detection};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

pub struct Mea {
    pub name: String,
    pub active_electrodes: Vec<String>,
}

#[derive(Default)]
pub struct Phase {
    pub sampling_frequency: f32,
    pub raw_data: HashMap<String, Vec<f32>>,
    pub peaks_trains: HashMap<String, (Vec<f32>, Vec<usize>)>,
    pub digitals: Vec<Vec<f32>>,
}

impl Phase {
    pub fn new() -> Phase {
        Phase::default()
    }

    pub fn compute_peak_train(&mut self, label: &str) -> Option<()> {
        if self.raw_data.contains_key(label) {
            let signal = &self.raw_data[label];
            if let Ok(threshold) =
                compute_threshold(&signal[..], self.sampling_frequency, 8 as _)
            {
                let peaks_train = spike_detection(
                    &signal[..],
                    self.sampling_frequency,
                    threshold,
                    2e-3,
                    2e-3,
                )?;
                self.peaks_trains.insert(label.to_string(), peaks_train);
                return Some(());
            }
        }
        None
    }

    pub fn compute_all_peak_trains(&mut self) -> Option<()> {
        for (label, signal) in &self.raw_data {
            if let Ok(threshold) =
                compute_threshold(&signal[..], self.sampling_frequency, 8 as _)
            {
                let peaks_train = spike_detection(
                    &signal[..],
                    self.sampling_frequency,
                    threshold,
                    2e-3,
                    2e-3,
                )?;
                self.peaks_trains.insert(label.clone(), peaks_train);
            } else {
                return None;
            }
        }

        Some(())
    }
}

#[allow(unused_must_use)]
impl Debug for Phase {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(formatter, "{{");
        writeln!(formatter, "Sampling frequency: {}", self.sampling_frequency);
        writeln!(formatter, "Digitals:");
        for (i, digital) in self.digitals.iter().enumerate() {
            writeln!(formatter, "\tdigital_{}: n_samples = {}", i, digital.len());
        }
        writeln!(formatter, "Raw Data:");
        for (label, data) in &self.raw_data {
            writeln!(formatter, "\t{}: n_samples = {}", label, data.len());
        }
        writeln!(formatter, "Peak trains:");
        for (label, (data_vals, data_times)) in &self.peaks_trains {
            writeln!(formatter, "\t{}: values n_points = {}, times n_points = {}",
                     label, data_vals.len(), data_times.len());
        }
        writeln!(formatter, "}}");
        Ok(())
    }
}

pub struct Recording {
    pub phases: Vec<Phase>,
}
