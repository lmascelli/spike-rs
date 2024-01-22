use crate::core::operations::{compute_threshold, spike_detection};
use crate::core::types::Signal;
use std::collections::HashMap;

pub struct Mea {
    pub name: String,
    pub active_electrodes: Vec<String>,
}

// #[allow(unused)]
// pub enum PhaseType {
//     Basal,
//     Stimulation,
// }

pub struct Phase {
    //     pub phase_type:         PhaseType,
    pub raw_datas: HashMap<String, Signal>,
    pub peak_trains: HashMap<String, Vec<usize>>,
    //     pub digital:            Option<Signal>,
}

impl Phase {
    pub fn new() -> Phase {
        Phase {
            raw_datas: HashMap::new(),
            peak_trains: HashMap::new(),
        }
    }

    pub fn compute_peak_train(&mut self, label: &str) -> Option<()> {
        if self.raw_datas.contains_key(label) {
            let signal = &self.raw_datas[label];
            if let Ok(threshold) =
                compute_threshold(&signal.data[..], signal.sampling_frequency, 8 as _)
            {
                let peak_train = spike_detection(
                    &signal.data[..],
                    signal.sampling_frequency,
                    threshold,
                    2e-3,
                    2e-3,
                )?;
                self.peak_trains.insert(label.to_string(), peak_train);
                return Some(());
            }
        }
        None
    }

    pub fn compute_all_peak_trains(&mut self) -> Option<()> {
        for (label, signal) in &self.raw_datas {
            if let Ok(threshold) =
                compute_threshold(&signal.data[..], signal.sampling_frequency, 8 as _)
            {
                let peak_train = spike_detection(
                    &signal.data[..],
                    signal.sampling_frequency,
                    threshold,
                    2e-3,
                    2e-3,
                )?;
                self.peak_trains.insert(label.clone(), peak_train);
            } else {
                return None;
            }
        }

        Some(())
    }
}

pub struct Recording {
    pub phases: Vec<Phase>,
}
