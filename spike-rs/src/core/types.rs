use crate::core::operations::{check_valid_bin_size, compute_threshold, spike_detection,
                              subsample_range, get_digital_intervals};
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
    pub el_stim_intervals: Vec<Vec<u64>>,
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

    pub fn psth(&self, bin_size: usize,
                       digital_index: usize) -> Result<Vec<Vec<usize>>, String> {
        
        if digital_index >= self.digitals.len() {
            return Err("Phase.psth: digital_index out of bounds of digitals Vec".to_string());
        }
        let stim_intervals = get_digital_intervals(&self.digitals[digital_index][..]);
        let channel_histos = self.get_subsampled_pre_stim_post_from_intervals(
            &stim_intervals, bin_size);

        let mut n_intervals = 0;
        let mut max_pre = 0;
        let mut max_stim = 0;
        let mut max_post = 0;

        for (_, intervals) in &channel_histos {
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
        ret.resize(n_intervals, vec![0;max_pre+max_stim+max_post]);
        for (i, (_, intervals)) in channel_histos.iter().enumerate() {
            for (pre, stim, post) in intervals {

                for (j, val) in pre.iter().enumerate() {
                    ret[i][j] += val;
                }
                for (j, val) in stim.iter().enumerate() {
                    ret[i][j+max_pre] += val;
                }
                for (j, val) in post.iter().enumerate() {
                    ret[i][j+max_pre+max_stim] += val;
                }
            }
        }
        Ok(ret)
    }

    pub fn get_subsampled_pre_stim_post_from_intervals(&self,
                                                       intervals: &Vec<(usize, usize)>,
                                                       bin_size: usize
                        ) -> HashMap<String, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>> 
    {
        let n_intervals = intervals.len();
        let raw_data_len = self.raw_data[self.raw_data.keys().collect::<Vec<&String>>()[0]].len(); 
        assert!(n_intervals != 0, "No intervals provided!!!");

        // adjust the bin_size
        let bin_size = check_valid_bin_size(intervals[0], bin_size);

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
                data_len = intervals[i].0 - intervals[i-1].1;
                n_pre =  data_len / bin_size;
                start_pre = intervals[i].0 - n_pre * bin_size;

                // stim
                data_len = intervals[i].1 - intervals[i].0;
                start_stim = intervals[i].0;
                n_stim =  data_len / bin_size;

                // post
                data_len = intervals[i+1].0 - intervals[i].1;
                start_post = intervals[i].1;
                n_post = data_len / bin_size;

            } else if i == 0 {
                // pre
                data_len = intervals[i].0;
                n_pre =  data_len / bin_size;
                start_pre = intervals[i].0 - n_pre * bin_size;

                // stim
                data_len = intervals[i].1 - intervals[i].0;
                start_stim = intervals[i].0;
                n_stim =  data_len / bin_size;

                // post
                data_len = intervals[i+1].0 - intervals[i].1;
                start_post = intervals[i].1;
                n_post = data_len / bin_size;

            } else {
                // pre
                data_len = intervals[i].0 - intervals[i-1].1;
                n_pre =  data_len / bin_size;
                start_pre = intervals[i].0 - n_pre * bin_size;

                // stim
                data_len = intervals[i].1 - intervals[i].0;
                start_stim = intervals[i].0;
                n_stim =  data_len / bin_size;

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
