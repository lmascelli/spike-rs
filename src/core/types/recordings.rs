use std::collections::HashMap;
use crate::core::types::Signal;

pub struct Mea {
    pub name: String,
    pub active_electrodes: Vec<String>,
}

#[allow(unused)]
pub enum PhaseType {
    Basal,
    Stimulation,
}

pub struct Phase {
    pub phase_type:         PhaseType,
    pub raw_datas:          HashMap<String, Signal>,
    pub peak_trains:        HashMap<String, Vec<usize>>,
    pub digital:            Option<Signal>,
}

pub struct Recording {
    pub phases:             Vec<Phase>,
}
