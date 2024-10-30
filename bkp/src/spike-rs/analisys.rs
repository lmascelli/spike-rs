use std::collections::HashMap;

use crate::{
    error::SpikeError,
    operations::{
        compute_threshold, get_digital_intervals, spike_detection,
        subsample_range,
    },
    types::PhaseHandler,
};

pub fn compute_peak_train(
    phase: &mut impl PhaseHandler,
    label: &str,
    start: Option<usize>,
    end: Option<usize>,
) -> Result<(), SpikeError> {
    let signal = phase.raw_data(label, start, end)?;
    let threshold =
        compute_threshold(&signal[..], phase.sampling_frequency(), 8 as _)?;
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

/// Count the number of spikes grouped for the duration of `bin_size`
/// before, during and after the stimulus interval
pub fn get_subsampled_pre_stim_post_from_intervals(
    phase: &mut impl PhaseHandler,
    intervals: &[(usize, usize)],
    bin_size: usize,
) -> Result<
    HashMap<String, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>>,
    SpikeError,
> {
    let n_intervals = intervals.len();
    let raw_data_len = phase.datalen();
    assert!(n_intervals != 0, "No intervals provided!!!");

    let mut scan_intervals = vec![];

    // here we scan how much bins each interval contains
    // before, during and after the stimulus and find the starting
    // sample of each group (pre, stim, post)
    for i in 0..n_intervals {
        let start_pre;
        let n_pre;
        let start_stim;
        let n_stim;
        let start_post;
        let n_post;
        let mut data_len;
        // if it's not the first or the last interval
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
        }
        // if it's the first
        else if i == 0 {
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
        }
        // else it's the last
        else {
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

        scan_intervals
            .push((start_pre, n_pre, start_stim, n_stim, start_post, n_post));
    }

    let mut ret = HashMap::new();
    for label in phase.labels() {
        let (data_times, _) = phase.peak_train(&label, None, None)?;
        let mut current_ret = vec![];
        for interval in &scan_intervals {
            current_ret.push((
                subsample_range(
                    &data_times[..],
                    interval.0,
                    bin_size,
                    interval.1,
                ),
                subsample_range(
                    &data_times[..],
                    interval.2,
                    bin_size,
                    interval.3,
                ),
                subsample_range(
                    &data_times[..],
                    interval.4,
                    bin_size,
                    interval.5,
                ),
            ));
        }
        ret.insert(label.clone(), current_ret);
    }
    Ok(ret)
}

pub fn subsample_peak_trains(
    phase: &mut impl PhaseHandler,
    bin_size: usize,
    digital_index: usize,
) -> Result<
    HashMap<String, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>>,
    SpikeError,
> {
    if digital_index >= phase.n_digitals() {
        return Err(SpikeError::IndexOutOfRange);
    }
    let stim_intervals =
        get_digital_intervals(&phase.digital(digital_index, None, None)?[..]);
    get_subsampled_pre_stim_post_from_intervals(
        phase,
        &stim_intervals,
        bin_size,
    )
}

pub fn subsampled_post_stimulus_times(
    phase: &mut impl PhaseHandler,
    bin_size: usize,
    n_bins_post_stim: usize,
    digital_index: usize,
) -> Result<Vec<Vec<usize>>, SpikeError> {
    if digital_index >= phase.n_digitals() {
        return Err(SpikeError::IndexOutOfRange);
    }
    let n_samples_req = n_bins_post_stim * bin_size;
    //println!("N SAMPLES REQUIRED: {n_samples_req}");

    let stim_intervals =
        get_digital_intervals(&phase.digital(digital_index, None, None)?[..]);

    // used to keep just the stimulation data that not started before or after
    // the recording
    let mut valid_intervals = vec![];

    for interval in &stim_intervals {
        let interval_length = interval.1 - interval.0;
        //println!("interval: {interval:?} - interval_length: {}, n_samples: {}", interval_length, interval_length);
        if interval_length > n_samples_req {
            valid_intervals.push(interval);
        }
    }

    //println!("INTERVALS -> {valid_intervals:?}");

    let mut ret = vec![vec! {0; n_bins_post_stim}; valid_intervals.len()];

    for label in phase.labels() {
        let peaks_times = phase.peak_train(&label, None, None)?.0;

        if peaks_times.len() == 0 {
            return Ok(vec![]);
        } else {
            // assume that valid_intervals are sorted in increasing order
            // and peaks are sorted in increasing order too
            let mut sample_index = 0;
            let mut interval_index = 0;
            'cycle_intervals: loop {
                if interval_index == valid_intervals.len() {
                    break 'cycle_intervals;
                }
                let interval = valid_intervals[interval_index];

                for bin in 0..n_bins_post_stim {
                    let start = interval.0 + bin * bin_size;
                    let end = interval.0 + (bin + 1) * bin_size;
                    loop {
                        if sample_index >= peaks_times.len() {
                            break 'cycle_intervals;
                        }
                        if peaks_times[sample_index] > end {
                            break;
                        }
                        if peaks_times[sample_index] >= start {
                            ret[interval_index][bin] += 1;
                        }
                        sample_index += 1;
                    }
                }

                interval_index += 1;
            }
        }
    }

    Ok(ret)
}
