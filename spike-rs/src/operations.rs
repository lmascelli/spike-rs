use crate::error::SpikeError;
use std::result::Result;

pub mod math {
    pub fn mean(range: &[f32]) -> f32 {
        let mut sum = 0f32;
        range.iter().for_each(|x| {
            sum += x;
        });
        sum / (range.len() as f32)
    }

    pub fn stdev(range: &[f32]) -> f32 {
        let mut sum = 0f32;
        let _mean = mean(range);
        range.iter().for_each(|x| {
            sum += (x - _mean) * (x - _mean);
        });
        (sum / (range.len() as f32 - 1.0f32)).sqrt()
    }

    pub fn min(range: &[f32]) -> f32 {
        let mut min = range[0];
        for value in range {
            if *value < min {
                min = *value;
            }
        }
        min
    }

    pub fn max(range: &[f32]) -> f32 {
        let mut max = range[0];
        for value in range {
            if *value > max {
                max = *value;
            }
        }
        max
    }

    pub fn train(n: usize, step: usize, offset: usize) -> Vec<f32> {
        let mut ret = vec![0f32; step * n + offset];
        for i in 0..n {
            ret[i * step + offset] = 1f32;
        }
        ret
    }

    pub fn convolve(s1: &[f32], s2: &[f32]) -> Vec<f32> {
        let (signal, filter) = if s1.len() > s2.len() {
            (s1, s2)
        } else {
            (s2, s1)
        };
        let filter: Vec<f32> = filter.iter().rev().map(|x| *x).collect();
        let slen = signal.len();
        let flen = filter.len();

        let mut ret = vec![0f32; slen];

        // head
        for i in 0..flen {
            for j in 0..i {
                ret[i] += signal[j] * filter[j];
            }
        }
        // body
        for i in flen..(slen - flen) {
            for j in 0..flen {
                ret[i] += signal[i + j] * filter[j];
            }
        }
        // tail
        for i in (slen - flen)..slen {
            for j in flen..0 {
                ret[i] += signal[i + j] * filter[j];
            }
        }

        ret
    }
}

pub fn compute_threshold(
    range: &[f32],
    sampling_frequency: f32,
    multiplier: f32,
) -> Result<f32, SpikeError> {
    const WINDOW_DURATION_TIME: f32 = 200e-3; // s
    const START_THRESHOLD: f32 = 100e-6; // V

    let window_duration_sample: usize = (WINDOW_DURATION_TIME * sampling_frequency) as usize;
    let number_of_windows: usize = range.len() / window_duration_sample;
    let windows_distance: usize = range.len() / number_of_windows;

    if range.len() < (window_duration_sample * number_of_windows) {
        return Err(SpikeError::ComputeThresholdTooFewSamples(
            range.len(),
            window_duration_sample * number_of_windows,
        ));
    }

    let mut threshold = START_THRESHOLD;

    for i in 0..number_of_windows {
        let starting_point = windows_distance * i;
        let ending_point = starting_point + window_duration_sample;
        let new_threshold = math::stdev(&range[starting_point..ending_point]);

        if new_threshold < threshold {
            threshold = new_threshold;
        }
    }

    Ok(threshold * multiplier)
}

pub fn spike_detection(
    data: &[f32],
    sampling_frequency: f32,
    threshold: f32,
    peak_duration: f32,
    refractory_time: f32,
) -> Result<(Vec<f32>, Vec<usize>), SpikeError> {
    // TODO check if reserving space for the ret increases performances.
    let mut ret_values = Vec::new();
    let mut ret_times = Vec::new();

    const OVERLAP: usize = 5;
    let data_length = data.len();

    let peak_duration: usize = (peak_duration * sampling_frequency) as usize;
    let refractory_time: usize = (refractory_time * sampling_frequency) as usize;

    if data_length < 2 || data_length < peak_duration {
        return Err(SpikeError::SpikeDetectionTooFewSamples);
    }

    let mut index = 1usize;
    let mut interval;
    let mut in_interval_index;

    let mut peak_start_sample;
    let mut peak_start_value;
    let mut peak_end_sample;
    let mut peak_end_value;

    while index < data_length - 1 {
        // If a minimum or a maximum has been found ...
        if (data[index].abs() > data[index - 1].abs())
            && (data[index].abs() >= data[index + 1].abs())
        {
            // check if the end of the interval where to check for a spike excedes
            // the length of the signal and, eventually, set the interval to end
            // earlier.
            if index + peak_duration > data_length {
                interval = data_length - index - 1;
            } else {
                interval = peak_duration;
            }

            // temporarely set the start of the spike to be at the current index
            peak_start_sample = index;
            peak_start_value = data[index];

            // look for minimum if the start value of the peak is positive
            if peak_start_value > 0f32 {
                peak_end_sample = index + 1;
                peak_end_value = peak_start_value;

                // find the minimum in [index, index+interval]
                in_interval_index = index + 1;
                while in_interval_index < index + interval {
                    if data[in_interval_index] < peak_end_value {
                        peak_end_sample = in_interval_index;
                        peak_end_value = data[in_interval_index];
                    }
                    in_interval_index += 1;
                } // end find minimum

                // find the actual maximum in [index, peak_end_sample]
                in_interval_index = index + 1;
                while in_interval_index < peak_end_sample {
                    if data[in_interval_index] > peak_start_value {
                        peak_start_sample = in_interval_index;
                        peak_start_value = data[in_interval_index];
                    }
                    in_interval_index += 1;
                } // end looking for actual maximum

                // if the minimum has been found at the boundary of the interval
                // check if the signal is still decreasing and look for the interval in
                // [index + interval, index + interval + OVERLAP] if this value does not
                // overcome the data_length
                if peak_end_sample == index + interval && index + interval + OVERLAP < data_length {
                    in_interval_index = peak_end_sample + 1;
                    while in_interval_index < index + interval + OVERLAP {
                        if data[in_interval_index] < peak_end_value {
                            peak_end_sample = in_interval_index;
                            peak_end_value = data[in_interval_index];
                        }
                        in_interval_index += 1;
                    }
                }
            }
            // end minimum branch
            else {
                // else look for a maximum
                peak_end_sample = index + 1;
                peak_end_value = peak_start_value;

                // find the maximum in [index, index+interval]
                in_interval_index = index + 1;
                while in_interval_index < index + interval {
                    if data[in_interval_index] > peak_end_value {
                        peak_end_sample = in_interval_index;
                        peak_end_value = data[in_interval_index];
                    }
                    in_interval_index += 1;
                } // end find maximum

                // find the actual minimum in [index, peak_end_sample]
                in_interval_index = index + 1;
                while in_interval_index < peak_end_sample {
                    if data[in_interval_index] < peak_start_value {
                        peak_start_sample = in_interval_index;
                        peak_start_value = data[in_interval_index];
                    }
                    in_interval_index += 1;
                } // end looking for actual minimum

                // if the maximum has been found at the boundary of the interval
                // check if the signal is still increasing and look for the interval in
                // [index + interval, index + interval + OVERLAP] if this value does not
                // overcome the data_length
                if peak_end_sample == index + interval && index + interval + OVERLAP < data_length {
                    in_interval_index = peak_end_sample + 1;
                    while in_interval_index < index + interval + OVERLAP {
                        if data[in_interval_index] > peak_end_value {
                            peak_end_sample = in_interval_index;
                            peak_end_value = data[in_interval_index];
                        }
                        in_interval_index += 1;
                    }
                }
            }

            // check if the difference overtakes the threshold
            let difference = peak_start_value - peak_end_value;

            if difference.abs() >= threshold {
                let (last_peak_val, last_peak_time) =
                    if peak_start_value.abs() > peak_end_value.abs() {
                        (peak_start_value, peak_start_sample)
                    } else {
                        (peak_end_value, peak_end_sample)
                    };

                ret_values.push(last_peak_val);
                ret_times.push(last_peak_time);

                // set the new index where to start looking for a peak
                if last_peak_time + refractory_time > peak_end_sample
                    && last_peak_time + refractory_time < data_length
                {
                    index = last_peak_time + refractory_time;
                } else {
                    index = peak_end_sample + 1;
                }

                continue;
            } // end threshold check
        }
        index += 1;
    }
    Ok((ret_values, ret_times))
}

/// Build an histogram of `n_bins` equidistant values containing the distribution of
/// the magnitude of the peaks. Returns the built histogram and the minimum and
/// maximum values found
///
/// # Arguments
///
/// * `range` -  the sequence of the value of the peaks
/// * `n_bins` - number of bins of the histogram
pub fn get_peaks_bins(range: &[f32], n_bins: usize) -> Option<(Vec<usize>, f32, f32)> {
    if n_bins == 0 {
        return None;
    }

    let mut ret = (vec![0; n_bins + 1], 0f32, 0f32);

    let range_mod: Vec<f32> = range.iter().map(|x| x.abs()).collect();

    let min = math::min(&range_mod[..]);
    let max = math::max(&range_mod[..]);
    ret.1 = min;
    ret.2 = max;
    let bin_size = (max - min) / n_bins as f32;
    if bin_size == 0f32 {
        None
    } else {
        for value in range_mod {
            let index = ((value - min) / bin_size) as usize;
            ret.0[index] += 1;
        }
        Some(ret)
    }
}

/// Build a sequence of couples of values (start, end) from a digital signal
/// representing the boundaries of active periods
///
/// # Arguments
///
/// * `digital` - the digital signal
pub fn get_digital_intervals(digital: &[f32]) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    let mut start = 0usize;
    let mut in_interval = false;

    for (i, value) in digital.iter().enumerate() {
        if in_interval {
            if *value == 0f32 {
                ret.push((start, i));
                in_interval = false;
            }
        } else if *value != 0f32 {
            start = i;
            in_interval = true;
        }
    }

    if in_interval {
        ret.push((start, digital.len()));
    }
    ret
}

/// Subsample the given range returning a vector with the number of spikes in
/// each bin. The input range shoulds contain the times of each peak detected
/// sorted in an increasing order
///
/// # Arguments
/// * `range` -           a range that contains the times of the peaks
/// * `starting_sample` - the starting value of the samples range
/// * `bin_size` -        the length of each bin
/// * `n_bins` -          then total number of bins
pub fn subsample_range(
    peak_times: &[usize],
    starting_sample: usize,
    bin_size: usize,
    n_bins: usize,
) -> Vec<usize> {
    let mut ret = vec![0; n_bins];
    let mut current_bin_index = 0;
    let mut current_bin_start = starting_sample;
    let mut current_bin_end = current_bin_start + bin_size;
    for peak in peak_times {
        if *peak < current_bin_start {
            // we are not in a useful bin yet
            continue;
        } else if *peak >= current_bin_start && *peak < current_bin_end {
            // we are in a useful bin
            ret[current_bin_index] += 1;
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
                    ret[current_bin_index] += 1;
                    break;
                }
            }
        }
    }
    ret
}
