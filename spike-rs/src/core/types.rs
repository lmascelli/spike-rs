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

    pub fn compute_all_peak_trains(&mut self,
                                   peak_duration: f32,
                                   refractory_time: f32,
                                   n_devs: f32,
                                   ) -> Option<()> {
        for (label, signal) in &self.raw_data {
            if let Ok(threshold) =
                compute_threshold(&signal[..], self.sampling_frequency, n_devs)
            {
                let peaks_train = spike_detection(
                    &signal[..],
                    self.sampling_frequency,
                    threshold,
                    peak_duration,
                    refractory_time,
                )?;
                self.peaks_trains.insert(label.clone(), peaks_train);
            } else {
                return None;
            }
        }

        Some(())
    }

    pub fn clear_peaks_over_threshold(&mut self, threshold: f32) {
        let peaks_trains = &mut self.peaks_trains as *mut HashMap<String,
                                                                  (Vec<f32>, Vec<usize>)>;
        let peaks_trains1 = unsafe {&mut *peaks_trains};
        let peaks_trains2 = unsafe {&mut *peaks_trains};
        for (label, (peaks_values, peaks_times)) in peaks_trains1 {
            let mut new_peaks_values = vec![];
            let mut new_peaks_times = vec![];
            for (i, value) in peaks_values.iter().enumerate() {
                if value.abs() < threshold {
                    new_peaks_values.push(*value);
                    new_peaks_times.push(peaks_times[i]);
                }
            }
            peaks_trains2.insert(label.clone(), (new_peaks_values, new_peaks_times));
        }
    }

    pub fn get_peaks_in_interval(&self, interval: &(usize, usize)) -> HashMap::<String, (Vec<f32>, Vec<usize>)> {
        let mut ret = HashMap::new();
        for (label, (values, times)) in &self.peaks_trains {
            let mut ret_values = vec![];
            let mut ret_times = vec![];
            let mut peak_counter = 0usize;
            loop {
                if times[peak_counter] >= interval.0 {
                    if times[peak_counter] <= interval.1 {
                        ret_values.push(values[peak_counter]);
                        ret_times.push(times[peak_counter]);
                    } else {
                        break;
                    }
                    peak_counter += 1;
                }
            }
            ret.insert(label.clone(), (ret_values, ret_times));
        }
        ret
    }

    pub fn get_peaks_in_consecutive_intervals(&self, intervals: &Vec<(usize, usize)>
    ) -> HashMap::<String, (Vec<f32>, Vec<usize>)> {
        // assume that the peaks are consecutives and non overlapped
        // also the peaks are supposed to be consecutives
        let mut ret = HashMap::new();

        for (label, (values, times)) in &self.peaks_trains {
            let mut ret_values = vec![];
            let mut ret_times = vec![];
            let mut interval_counter = 0usize;
            let mut peak_counter = 0usize;
            loop {

                if times[peak_counter] >= intervals[interval_counter].0 {
                    if times[peak_counter] <= intervals[interval_counter].1 {
                        ret_values.push(values[peak_counter]);
                        ret_times.push(times[peak_counter]);
                    } else {
                        interval_counter += 1;
                    }
                }

                if interval_counter == intervals.len() {
                    break;
                } else {
                    peak_counter += 1;
                }
            }
            ret.insert(label.clone(), (ret_values, ret_times));
        }
         
        ret
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
