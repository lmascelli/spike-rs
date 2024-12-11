use crate::{
    analysis::{
        digital::get_digital_intervals, spike_analysis::get_subsampled_pre_stim_post_from_intervals,
    },
    error::SpikeError,
    types::PhaseHandler,
};
use std::collections::HashMap;

/// Subsample the given range returning a vector with the number of spikes in
/// each bin. The input range shoulds contain the times of each peak detected
/// sorted in an increasing order
///
/// # Arguments
/// * `peak_times` -      a range that contains the times of the peaks
/// * `starting_sample` - the starting value of the samples range
/// * `bin_size` -        the length of each bin
/// * `n_bins` -          then total number of bins
pub fn subsample_range(
    peak_times: &[usize],
    starting_sample: usize,
    bin_size: usize,
    n_bins: usize,
) -> Vec<usize> {
    let mut ret = vec![0; n_bins as usize];
    let mut current_bin_index = 0;
    let mut current_bin_start = starting_sample;
    let mut current_bin_end = current_bin_start + bin_size;
    for peak in peak_times {
        if *peak < current_bin_start {
            // we are not in a useful bin yet
            continue;
        } else if *peak >= current_bin_start && *peak < current_bin_end {
            // we are in a useful bin
            ret[current_bin_index as usize] += 1;
        } else {
            // we've overcome the current bin. need to find the index of the following bin
            loop {
                current_bin_index += 1; // advance to the next bin
                if current_bin_index == n_bins {
                    // if we reached the end of the bins return
                    return ret;
                }
                current_bin_start += bin_size;
                current_bin_end += bin_size;
                if *peak >= current_bin_start && *peak < current_bin_end {
                    // check if the peak is
                    // contained in this bin
                    ret[current_bin_index as usize] += 1;
                    break;
                }
            }
        }
    }
    ret
}

pub fn subsample_peak_trains(
    phase: &mut impl PhaseHandler,
    bin_size: usize,
    digital_index: usize,
) -> Result<HashMap<String, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>>, SpikeError> {
    if digital_index >= phase.n_digitals() {
        return Err(SpikeError::IndexOutOfRange);
    }
    let stim_intervals = get_digital_intervals(&phase.digital(digital_index, None, None)?[..]);
    get_subsampled_pre_stim_post_from_intervals(phase, &stim_intervals, bin_size)
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

    let stim_intervals = get_digital_intervals(&phase.digital(digital_index, None, None)?[..]);

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
