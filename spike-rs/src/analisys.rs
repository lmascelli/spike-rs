use std::collections::HashMap;

use crate::{
    error::SpikeError,
    operations::{compute_threshold, get_digital_intervals, spike_detection, subsample_range},
    types::PhaseHandler,
};

pub fn compute_peak_train(
    phase: &mut impl PhaseHandler,
    label: &str,
    start: Option<usize>,
    end: Option<usize>,
) -> Result<(), SpikeError> {
    let signal = phase.raw_data(label, start, end)?;
    let threshold = compute_threshold(&signal[..], phase.sampling_frequency(), 8 as _)?;
    let peaks_train = spike_detection(
        &signal[..],
        phase.sampling_frequency(),
        threshold,
        2e-3,
        2e-3,
    )?;
    phase.set_peak_train(label, start, end, peaks_train)?;
    Ok(())
}

pub fn get_subsampled_pre_stim_post_from_intervals(
    phase: &mut impl PhaseHandler,
    intervals: &[(usize, usize)],
    bin_size: usize,
) -> Result<HashMap<String, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>>, SpikeError> {
    let n_intervals = intervals.len();
    let raw_data_len = phase.datalen();
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
    for label in phase.labels() {
        let (data_times, _) = phase.peak_train(&label, None, None)?;
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
    Ok(ret)
}

pub fn psth(
    phase: &mut impl PhaseHandler,
    bin_size: usize,
    digital_index: usize,
) -> Result<Vec<Vec<usize>>, SpikeError> {
    if digital_index >= phase.n_digitals() {
        return Err(SpikeError::IndexOutOfRange);
    }
    let stim_intervals = get_digital_intervals(&phase.digital(digital_index, None, None)?[..]);
    let channel_histos =
        get_subsampled_pre_stim_post_from_intervals(phase, &stim_intervals, bin_size)?;

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

/* TODO
pub fn clear_peaks_over_threshold(phase: &mut impl PhaseHandler, threshold: f32) {
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
 * */
