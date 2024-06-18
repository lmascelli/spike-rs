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
        end: Option<usize>,
        data: &[f32],
    ) -> Result<(), SpikeError> {
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

            let data = &self.peaks_trains[channel];
            todo!()
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

            let (original_vals, original_samples) = self.peaks_trains.get_mut(channel).unwrap();

            todo!();
            Ok(())
        } else {
            return Err(SpikeError::LabelNotFound);
        }
    }
}

#[allow(clippy::type_complexity)]
impl PhaseMemory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extract_range(&self, start: usize, end: usize) -> Option<Self> {
        if self.raw_data.len() == 0 {
            eprintln!("the phase has no data inside");
            None
        } else {
            let data_len = self.raw_data.iter().map(|(_k, v)| v.len()).next().unwrap();
            if start > end {
                eprintln!(
                    "the start of the range is greater than the end: {}, {}",
                    start, end
                );
                None
            } else if end >= data_len {
                eprintln!(
                    "the end of the range is greater than length of the signals: {}, {}",
                    end, data_len
                );
                None
            } else {
                let mut ret = Self::default();
                ret.sampling_frequency = self.sampling_frequency;
                for (label, signal) in &self.raw_data {
                    ret.raw_data.insert(
                        label.clone(),
                        signal[start..end].iter().map(|v| *v).collect(),
                    );
                }
                for digital in &self.digitals {
                    ret.digitals
                        .push(digital[start..end].iter().map(|x| *x).collect());
                }
                for (label, (opv, opt)) in &self.peaks_trains {
                    let mut i_start = 0;
                    let mut i_end = opt.len() - 1;
                    for i in 0..opt.len() {
                        if opt[i] >= start {
                            i_start = i;
                        }
                    }

                    for i in opt.len() - 1..0 {
                        if opt[i] <= end {
                            i_end = i;
                        }
                    }

                    ret.peaks_trains.insert(
                        label.clone(),
                        (
                            opv[i_start..=i_end].iter().map(|v| *v).collect(),
                            opt[i_start..=i_end].iter().map(|v| *v).collect(),
                        ),
                    );

                    ret.el_stim_intervals = self.el_stim_intervals.clone();
                }
                Some(ret)
            }
        }
    }

    pub fn compute_peak_train(&mut self, label: &str) -> Option<()> {
        if self.raw_data.contains_key(label) {
            let signal = &self.raw_data[label];
            if let Ok(threshold) = compute_threshold(&signal[..], self.sampling_frequency, 8 as _) {
                let peaks_train =
                    spike_detection(&signal[..], self.sampling_frequency, threshold, 2e-3, 2e-3)?;
                self.peaks_trains.insert(label.to_string(), peaks_train);
                return Some(());
            }
        }
        None
    }

    pub fn compute_all_peak_trains(
        &mut self,
        peak_duration: f32,
        refractory_time: f32,
        n_devs: f32,
    ) -> Option<()> {
        for (label, signal) in &self.raw_data {
            if let Ok(threshold) = compute_threshold(&signal[..], self.sampling_frequency, n_devs) {
                // println!("{} -> {}", label, threshold);
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
        let peaks_trains = &mut self.peaks_trains as *mut HashMap<String, (Vec<f32>, Vec<usize>)>;
        let peaks_trains1 = unsafe { &mut *peaks_trains };
        let peaks_trains2 = unsafe { &mut *peaks_trains };
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

    pub fn get_peaks_in_interval(
        &self,
        interval: &(usize, usize),
    ) -> HashMap<String, (Vec<f32>, Vec<usize>)> {
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

    pub fn get_peaks_in_consecutive_intervals(
        &self,
        intervals: &[(usize, usize)],
    ) -> HashMap<String, (Vec<f32>, Vec<usize>)> {
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

    pub fn get_subsampled_pre_stim_post_from_intervals(
        &self,
        intervals: &[(usize, usize)],
        bin_size: usize,
    ) -> HashMap<String, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>> {
        let n_intervals = intervals.len();
        let raw_data_len = self.raw_data[self.raw_data.keys().collect::<Vec<&String>>()[0]].len();
        assert!(n_intervals != 0, "No intervals provided!!!");

        // println!("n_intervals: {n_intervals}");
        // println!("raw_data_len: {raw_data_len}");
        // println!("bin_size: {bin_size}");

        // get the intervals to subsample
        let mut scan_intervals = vec![];

        for i in 0..n_intervals {
            let start_pre;
            let n_pre;
            let start_stim;
            let n_stim;
            let start_post;
            let n_post;
            let mut data_len;
            if i != 0 && i != n_intervals - 1 {
                // pre
                data_len = intervals[i].0 - intervals[i - 1].1;
                n_pre = data_len / bin_size;
                start_pre = intervals[i].0 - n_pre * bin_size;

                // stim
                data_len = intervals[i].1 - intervals[i].0;
                start_stim = intervals[i].0;
                n_stim = data_len / bin_size;

                // post
                data_len = intervals[i + 1].0 - intervals[i].1;
                start_post = intervals[i].1;
                n_post = data_len / bin_size;
            } else if i == 0 {
                // pre
                data_len = intervals[i].0;
                n_pre = data_len / bin_size;
                start_pre = intervals[i].0 - n_pre * bin_size;

                // stim
                data_len = intervals[i].1 - intervals[i].0;
                start_stim = intervals[i].0;
                n_stim = data_len / bin_size;

                // post
                data_len = intervals[i + 1].0 - intervals[i].1;
                start_post = intervals[i].1;
                n_post = data_len / bin_size;
            } else {
                // pre
                data_len = intervals[i].0 - intervals[i - 1].1;
                n_pre = data_len / bin_size;
                start_pre = intervals[i].0 - n_pre * bin_size;

                // stim
                data_len = intervals[i].1 - intervals[i].0;
                start_stim = intervals[i].0;
                n_stim = data_len / bin_size;

                // post
                data_len = raw_data_len - intervals[i].1;
                start_post = intervals[i].1;
                n_post = data_len / bin_size;
            }

            scan_intervals.push((start_pre, n_pre, start_stim, n_stim, start_post, n_post));
        }

        let mut ret = HashMap::new();
        for (label, (_, data_times)) in &self.peaks_trains {
            let mut current_ret = vec![];
            for interval in &scan_intervals {
                current_ret.push((
                    subsample_range(&data_times[..], interval.0, bin_size, interval.1),
                    subsample_range(&data_times[..], interval.2, bin_size, interval.3),
                    subsample_range(&data_times[..], interval.4, bin_size, interval.5),
                ));
            }
            ret.insert(label.clone(), current_ret);
        }
        ret
    }

    pub fn psth(&self, bin_size: usize, digital_index: usize) -> Result<Vec<Vec<usize>>, String> {
        if digital_index >= self.digitals.len() {
            return Err("Phase.psth: digital_index out of bounds of digitals Vec".to_string());
        }
        let stim_intervals = get_digital_intervals(&self.digitals[digital_index][..]);
        let channel_histos =
            self.get_subsampled_pre_stim_post_from_intervals(&stim_intervals, bin_size);

        let mut n_intervals = 0;
        let mut max_pre = 0;
        let mut max_stim = 0;
        let mut max_post = 0;

        for intervals in channel_histos.values() {
            n_intervals = intervals.len();
            for (pre, stim, post) in intervals {
                if pre.len() > max_pre {
                    max_pre = pre.len();
                }
                if stim.len() > max_stim {
                    max_stim = stim.len();
                }
                if post.len() > max_post {
                    max_post = post.len();
                }
            }
        }

        let mut ret = vec![];
        ret.resize(n_intervals, vec![0; max_pre + max_stim + max_post]);
        for (i, (_, intervals)) in channel_histos.iter().enumerate() {
            for (pre, stim, post) in intervals {
                for (j, val) in pre.iter().enumerate() {
                    ret[i][j] += val;
                }
                for (j, val) in stim.iter().enumerate() {
                    ret[i][j + max_pre] += val;
                }
                for (j, val) in post.iter().enumerate() {
                    ret[i][j + max_pre + max_stim] += val;
                }
            }
        }
        Ok(ret)
    }
}

impl std::fmt::Display for Phase {
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
