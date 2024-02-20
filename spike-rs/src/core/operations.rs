use std::result::Result;

pub mod math {
    pub fn mean(range: &[f32]) -> f32 {
        let mut sum = 0f32;
        range.iter().for_each(|x| {
            sum += x;
        });
        let ret = sum / (range.len() as f32);
        ret
    }

    pub fn stdev(range: &[f32]) -> f32 {
        let mut sum = 0f32;
        let _mean = mean(range);
        range.iter().for_each(|x| {
            sum += (x - _mean) * (x - _mean);
        });
        let ret = (sum / (range.len() as f32 - 1.0f32)).sqrt();
        ret
    }

    pub fn min(range: &[f32]) -> f32 {
        let mut min = range[0];
        for value in range {
            if *value < min {
                min = *value;
            }
        }
        return min;
    }
    
    pub fn max(range: &[f32]) -> f32 {
        let mut max = range[0];
        for value in range {
            if *value > max {
                max = *value;
            }
        }
        return max;
    }
}

pub fn compute_threshold(
    range: &[f32],
    sampling_frequency: f32,
    multiplier: f32,
) -> Result<f32, String> {
    const NUMBER_OF_WINDOWS: usize = 30;
    const WINDOW_DURATION_TIME: f32 = 200e-3; // s
    const START_THRESHOLD: f32 = 100e-6; // V

    let window_duration_sample: usize = (WINDOW_DURATION_TIME * sampling_frequency) as usize;
    let windows_distance: usize = range.len() / NUMBER_OF_WINDOWS;

    if range.len() < (window_duration_sample * NUMBER_OF_WINDOWS) {
        return Err(format!(
            "compute_threshold: too few samples ({}) to
        automatically compute threshold; needed at least {}",
            range.len(),
            window_duration_sample * NUMBER_OF_WINDOWS
        ));
    }

    let mut threshold = START_THRESHOLD;

    for i in 0..NUMBER_OF_WINDOWS {
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
) -> Option<(Vec<f32>, Vec<usize>)> {
    
    // TODO check if reserving space for the ret increases performances.
    let mut ret_values = Vec::new();
    let mut ret_times = Vec::new();

    const OVERLAP: usize = 5;
    let data_length = data.len();

    let peak_duration: usize = (peak_duration * sampling_frequency) as usize;
    let refractory_time: usize = (refractory_time * sampling_frequency) as usize;

    if data_length < 2 || data_length < peak_duration {
        eprintln!("spike_detection: ERROR too few samples provided");
        return None;
    }

    let mut index = 2usize;
    let mut new_index = 1usize;
    let mut interval;
    let mut in_interval_index;

    let mut peak_start_sample;
    let mut peak_start_value;
    let mut peak_end_sample;
    let mut peak_end_value;

    while index < data_length - 1 {
        if index < new_index {
            index += 1;
            continue;
        }


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
                // println!("Minimum search");

                peak_end_sample = index + 1;
                peak_end_value = peak_start_value;

                // find the minimum
                in_interval_index = index + 1;
                while in_interval_index < index + interval {
                    if data[in_interval_index] < peak_end_value {
                        peak_end_sample = in_interval_index;
                        peak_end_value = data[in_interval_index];
                    }

                    // find the actual maximum in the interval before the minimum
                    let mut inner_interval_index = in_interval_index;
                    while inner_interval_index < peak_end_sample {
                        if data[inner_interval_index] > peak_start_value {
                            peak_start_sample = inner_interval_index;
                            peak_start_value = data[inner_interval_index];
                        }

                        inner_interval_index += 1;
                    } // end looking for actual maximum

                    // TODO understand what's going on here
                    if peak_end_sample == index + interval
                        && index + interval + OVERLAP < data_length
                    {
                        let mut i = peak_end_sample + 1;
                        while i < index + interval + OVERLAP {
                            if data[i] < peak_end_value {
                                peak_end_sample = i;
                                peak_end_value = data[i];
                            }
                            i += 1;
                        }
                    }

                    in_interval_index += 1;
                } // end finding the actual minimum
            } // end maximum branch
            else { // else look for a maximum
                // println!("Maximum search");

                peak_end_sample = index + 1;
                peak_end_value = peak_start_value;

                // find the maximum
                in_interval_index = index + 1;
                while in_interval_index < index + interval {
                    if data[in_interval_index] >= peak_end_value {
                        peak_end_sample = in_interval_index;
                        peak_end_value = data[in_interval_index];
                    }

                    // find the actual minimum in the interval before the minimum
                    let mut inner_interval_index = in_interval_index;
                    while inner_interval_index < peak_end_sample {
                        if data[inner_interval_index] < peak_start_value {
                            peak_start_sample = inner_interval_index;
                            peak_start_value = data[inner_interval_index];
                        }

                        inner_interval_index += 1;
                    } // end looking for actual minimum

                    // TODO understand what's going on here
                    if peak_end_sample == index + interval
                        && index + interval + OVERLAP < data_length
                    {
                        let mut i = peak_end_sample + 1;
                        while i < index + interval + OVERLAP {
                            if data[i] > peak_end_value {
                                peak_end_sample = i;
                                peak_end_value = data[i];
                            }
                            i += 1;
                        }
                    }

                    in_interval_index += 1;
                } // end finding the actual maximum
            } // end minimum branch

            // check if the difference overtakes the threshold
            let difference = peak_start_value - peak_end_value;
            if difference.abs() >= threshold {
                let (last_peak_val, last_peak_time) = if difference > 0f32 {
                    (peak_start_value, peak_start_sample)
                } else {
                    (peak_end_value, peak_end_sample)
                };
                ret_values.push(last_peak_val);
                ret_times.push(last_peak_time);

                // set the new index where to start looking for a peak
                if last_peak_time + refractory_time > peak_end_sample &&
                    last_peak_time + refractory_time < data_length {
                        new_index = last_peak_time + refractory_time;
                    } else {
                        new_index = peak_end_sample + 1;
                    }
            } // end threshold check
        }

        index += 1;
    }
    Some((ret_values, ret_times))
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

    let mut ret = (vec![0; n_bins+1], 0f32, 0f32);

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
pub fn get_digital_intervals(digital: &[f32]) -> Option<Vec<(usize, usize)>> {
    let mut ret = vec![];
    let mut start = 0usize;
    let mut in_interval = false;

    for (i, value) in digital.iter().enumerate() {
        if in_interval {
            if *value == 0f32 {
                ret.push((start, i));
                in_interval = false;
            }
        } else {
            if *value != 0f32 {
                start = i;
                in_interval = true;
            }
        }
    }
    if in_interval {
        ret.push((start, digital.len()));
    }
    Some(ret)
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
pub fn subsample_range(peak_times: &[usize], starting_sample: usize, bin_size: usize, n_bins: usize) -> Vec<usize> {
    let mut ret = vec![0; n_bins];
    let mut current_bin_index = 0;
    let mut current_bin_start = starting_sample;
    let mut current_bin_end = current_bin_start + bin_size;
    for peak in peak_times {
        if *peak < current_bin_start { // we are not in a useful bin yet
            continue;
        }
        else if *peak >= current_bin_start && *peak < current_bin_end { // we are in a useful bin
            ret[current_bin_index] += 1;
        }
        else { // we've overcome the current bin. need to find the index of the following bin
            loop {
                current_bin_index += 1; // advance to the next bin
                if current_bin_index == n_bins { // if we reached the end of the bins return
                    return ret;
                }
                current_bin_start += bin_size;
                current_bin_end += bin_size;
                if *peak >= current_bin_start && *peak < current_bin_end { // check if the peak is
                    // contained in this bin
                    ret[current_bin_index] += 1;
                    break;
                }
            }
        }
    }
    return ret;
}

/// Check if a bin_size exactly divides an interval and suggest a near valid one if not.
pub fn check_valid_bin_size(interval: (usize, usize), bin_size: usize) -> usize {
    assert!(bin_size != 0, "bin_size should be greater than 0");

    const MAX_RESIDUE: i64 = 3;
    let interval_length = (interval.1 - interval.0) as i64;
    let n_bins = interval_length / bin_size as i64;
    let mut current_increment = 0i64;
    let mut sign = 1i64;
    let mut current_residue = interval_length - n_bins * (bin_size as i64 + sign * current_increment);
    while current_residue.abs() > MAX_RESIDUE {
        if sign < 0 {
            current_increment += 1;
        }
        sign *= -1;
        current_residue = interval_length - n_bins * (bin_size as i64 + current_increment * sign);
    }

    // println!("bin_size: {}\ncurrent_increment:{}\nsign:{}", bin_size, current_increment, sign);
    return bin_size + ( current_increment * sign ) as usize;
}
