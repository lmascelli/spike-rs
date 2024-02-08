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
                println!("Minimum search");

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
                println!("Maximum search");

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
