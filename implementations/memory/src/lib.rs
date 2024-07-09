use spike_rs::{error::SpikeError, types::PhaseHandler};
use std::collections::HashMap;

#[derive(Default)]
pub struct PhaseMemory {
    pub datalen: usize,
    pub sampling_frequency: f32,
    pub raw_data: HashMap<String, Vec<f32>>,
    pub peaks_trains: HashMap<String, (Vec<f32>, Vec<usize>)>,
    pub digitals: Vec<Vec<f32>>,
    pub el_stim_intervals: Vec<Vec<u64>>,
}

impl PhaseMemory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for PhaseMemory {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(formatter, "{{")?;
        writeln!(formatter, "Sampling frequency: {}", self.sampling_frequency)?;
        writeln!(formatter, "Digitals:")?;
        for (i, digital) in self.digitals.iter().enumerate() {
            writeln!(formatter, "\tdigital_{}: n_samples = {}", i, digital.len())?;
        }
        writeln!(formatter, "Raw Data:")?;
        for (label, data) in &self.raw_data {
            writeln!(formatter, "\t{}: n_samples = {}", label, data.len())?;
        }
        writeln!(formatter, "Peak trains:")?;
        for (label, (data_vals, data_times)) in &self.peaks_trains {
            writeln!(
                formatter,
                "\t{}: values n_points = {}, times n_points = {}",
                label,
                data_vals.len(),
                data_times.len()
            )?;
        }
        writeln!(formatter, "}}")?;
        Ok(())
    }
}

impl PhaseHandler for PhaseMemory {
    fn sampling_frequency(&self) -> f32 {
        self.sampling_frequency
    }

    fn labels(&self) -> Vec<String> {
        self.raw_data.keys().cloned().collect()
    }

    fn datalen(&self) -> usize {
        self.datalen
    }

    fn raw_data(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<Vec<f32>, SpikeError> {
        if self.raw_data.contains_key(channel) {
            let _start;
            let _end;
            if let Some(start) = start {
                _start = start;
            } else {
                _start = 0;
            }
            if let Some(end) = end {
                _end = end;
            } else {
                _end = self.datalen;
            }

            // check if ranges are in bounds
            if _start >= self.datalen || _start >= _end {
                return Err(SpikeError::IndexOutOfRange);
            }

            if _end >= self.datalen || _end >= _start {
                return Err(SpikeError::IndexOutOfRange);
            }
            // end check

            let data = &self.raw_data[channel];
            Ok(data[_start.._end].into())
        } else {
            return Err(SpikeError::LabelNotFound);
        }
    }

    fn set_raw_data(
        &mut self,
        channel: &str,
        start: Option<usize>,
        data: &[f32],
    ) -> Result<(), SpikeError> {
        if self.raw_data.contains_key(channel) {
            let _start;
            if let Some(start) = start {
                _start = start;
            } else {
                _start = 0;
            }
            // check if ranges are in bounds
            if _start >= self.datalen {
                return Err(SpikeError::IndexOutOfRange);
            }

            if _start + data.len() >= self.datalen {
                return Err(SpikeError::IndexOutOfRange);
            }
            // end check

            let original_data = self.raw_data.get_mut(channel).unwrap();
            for (i, val) in data.iter().enumerate() {
                original_data[i + _start] = *val;
            }

            Ok(())
        } else {
            return Err(SpikeError::LabelNotFound);
        }
    }

    fn n_digitals(&self) -> usize {
        self.digitals.len()
    }

    fn digital(
        &self,
        index: usize,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<Vec<f32>, SpikeError> {
        if index < self.digitals.len() {
            let _start;
            let _end;
            if let Some(start) = start {
                _start = start;
            } else {
                _start = 0;
            }
            if let Some(end) = end {
                _end = end;
            } else {
                _end = self.datalen;
            }

            // check if ranges are in bounds
            if _start >= self.datalen || _start >= _end {
                return Err(SpikeError::IndexOutOfRange);
            }

            if _end >= self.datalen || _end >= _start {
                return Err(SpikeError::IndexOutOfRange);
            }
            // end check

            let data = &self.digitals[index];
            Ok(data[_start.._end].into())
        } else {
            return Err(SpikeError::LabelNotFound);
        }
    }

    fn set_digital(
        &mut self,
        index: usize,
        start: Option<usize>,
        end: Option<usize>,
        data: &[f32],
    ) -> Result<(), SpikeError> {
        if index < self.digitals.len() {
            let _start;
            let _end;
            if let Some(start) = start {
                _start = start;
            } else {
                _start = 0;
            }
            if let Some(end) = end {
                _end = end;
            } else {
                _end = self.datalen;
            }

            // check if ranges are in bounds
            if data.len() != _end - _start {
                return Err(SpikeError::ReplaceRangeError);
            }
            if _start >= self.datalen || _start >= _end {
                return Err(SpikeError::IndexOutOfRange);
            }

            if _end >= self.datalen || _end >= _start {
                return Err(SpikeError::IndexOutOfRange);
            }
            // end check

            let original_data = self.digitals.get_mut(index).unwrap();
            for (i, val) in data.iter().enumerate() {
                original_data[i + _start] = *val;
            }

            Ok(())
        } else {
            return Err(SpikeError::IndexOutOfRange);
        }
    }

    fn n_events(&self) -> usize {
        self.el_stim_intervals.len()
    }

    fn events(&self, index: usize) -> Result<Vec<u64>, SpikeError> {
        if index < self.el_stim_intervals.len() {
            return Ok(self.el_stim_intervals[index].clone());
        } else {
            return Err(SpikeError::IndexOutOfRange);
        }
    }

    fn peak_train(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<(Vec<f32>, Vec<usize>), SpikeError> {
        if self.peaks_trains.contains_key(channel) {
            let _start;
            let _end;
            if let Some(start) = start {
                _start = start;
            } else {
                _start = 0;
            }
            if let Some(end) = end {
                _end = end;
            } else {
                _end = self.datalen;
            }

            // check if ranges are in bounds
            if _start >= self.datalen || _start >= _end {
                return Err(SpikeError::IndexOutOfRange);
            }

            if _end >= self.datalen || _end >= _start {
                return Err(SpikeError::IndexOutOfRange);
            }
            // end check

            let (vals, samples) = &self.peaks_trains[channel];
            let array_start = *(*samples)
                .iter()
                .find(|x| **x >= _start)
                .unwrap_or(&self.datalen);
            let array_end = *(*samples)
                .iter()
                .find(|x| **x <= _end)
                .unwrap_or(&self.datalen);
            Ok((
                vals[array_start..array_end].into(),
                samples[array_start..array_end].into(),
            ))
        } else {
            return Err(SpikeError::LabelNotFound);
        }
    }

    fn set_peak_train(
        &mut self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
        data: (Vec<f32>, Vec<usize>),
    ) -> Result<(), SpikeError> {
        if self.peaks_trains.contains_key(channel) {
            let _start;
            let _end;
            if let Some(start) = start {
                _start = start;
            } else {
                _start = 0;
            }
            if let Some(end) = end {
                _end = end;
            } else {
                _end = self.datalen;
            }

            // check if ranges are in bounds
            if data.0.len() != data.1.len() {
                return Err(SpikeError::ReplaceRangeError);
            }
            if _start >= self.datalen || _start >= _end {
                return Err(SpikeError::IndexOutOfRange);
            }

            if _end >= self.datalen || _end >= _start {
                return Err(SpikeError::IndexOutOfRange);
            }
            // end check

            let (vals, samples) = self.peaks_trains.get(channel).unwrap();
            let array_start = *(*samples)
                .iter()
                .find(|x| **x >= _start)
                .unwrap_or(&self.datalen);
            let array_end = *(*samples)
                .iter()
                .find(|x| **x <= _end)
                .unwrap_or(&self.datalen);

            let mut new_vals = vec![];
            let mut new_samples = vec![];

            new_vals.extend_from_slice(&vals[0..array_start]);
            new_samples.extend_from_slice(&samples[0..array_start]);
            new_vals.extend_from_slice(&data.0);
            new_samples.extend_from_slice(&data.1);
            new_vals.extend_from_slice(&vals[array_end..self.datalen]);
            new_samples.extend_from_slice(&samples[array_end..self.datalen]);

            self.peaks_trains
                .insert(channel.to_string(), (new_vals, new_samples));

            Ok(())
        } else {
            return Err(SpikeError::LabelNotFound);
        }
    }
}
