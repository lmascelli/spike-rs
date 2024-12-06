use super::error::SpikeError;
use super::operations::{
    compute_threshold, get_digital_intervals, spike_detection, subsample_range,
};
use super::types::PhaseHandler;
use std::collections::HashMap;

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

/// Count the number of spikes grouped for the duration of `bin_size`
/// before, during and after the stimulus interval
pub fn get_subsampled_pre_stim_post_from_intervals(
    phase: &mut impl PhaseHandler,
    intervals: &[(usize, usize)],
    bin_size: usize,
) -> Result<HashMap<String, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>>, SpikeError> {
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

/// Finds peaks in an histogram representing the density distriburion of ISI of a
/// peak train.
pub fn logisi_get_peaks(
    hist: &[f32],
    window_width_half: usize,
    threshold: f32,
    num_peaks: Option<usize>,
) -> (Vec<usize>, Vec<f32>) {
    let hist_len = hist.len();

    // if no num_peaks is provided the number of peaks to find is set to the lenght
    // of the hist array
    let num_peaks = num_peaks.unwrap_or(hist_len);

    let mut positions = vec![]; // location of the found peaks
    let mut peaks = vec![]; // values of the found peaks

    let mut j = 1; // index for hist array

    // cycle through the hist values excluding boundaries
    while j < hist_len - 1 && positions.len() <= num_peaks {
        // 1. set the boundary of the detection window

        // left boundary of the range. check that it's not before the start of the array
        let end_l = if j + window_width_half > hist_len {
            hist_len - window_width_half
        } else {
            j + window_width_half
        };

        // right boundary of the range. check that it's not after the end of the array
        let end_r = if j + window_width_half < hist_len {
            1
        } else {
            j - window_width_half
        };

        // 2. check if the jth value is greater than all the values in the window
        let mut is_peak = true;
        for i in end_l..j {
            if hist[i] + threshold > hist[j] {
                is_peak = false;
                break;
            }
        }

        for i in j + 1..end_r {
            if hist[i] + threshold > hist[j] {
                is_peak = false;
                break;
            }
        }

        if is_peak {
            positions.push(j);
            peaks.push(hist[j]);
        }

        // 3. adjust the next index
        if is_peak {
            j += 1 + window_width_half;
        } else {
            j += 1;
        }
    }

    (positions, peaks)
}

/// Compute the histogram of `data` within the ranges `bins`
pub fn hist<T>(data: &[T], bins: &[T]) -> Vec<usize>
where
    T: std::cmp::PartialOrd,
{
    let mut ret = vec![0; bins.len()];
    for val in data {
        for bin in 0..bins.len() - 1 {
            if *val >= bins[bin] && *val <= bins[bin + 1] {
                ret[bin] += 1;
                break;
            }
        }
    }
    ret
}

/// Calculate the cutoff threshold for burst detection
/// NOTE! the peaks are assumed to be consecutive and not duplicated
pub fn logisi_calc_threshold(
    peak_train: &[usize],
    sampling_frequency: f32,
) -> Result<f32, SpikeError> {
    // --------------------------------------------------------------------------
    // 1. get the differences in number of samples between adjacent peaks and
    // convert them in milliseconds
    let isi: Vec<f32> = super::operations::math::diff(peak_train)
        .iter()
        .map(|x| *x as f32 / sampling_frequency as f32 * 1000f32)
        .collect();

    let isi_len = isi.len();

    let max_isi = isi
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less))
        .expect("logisi_calc_threshold: Failed to find the ISI max")
        .clone()
        .ceil();

    let ranges = super::operations::math::logspace(0f32, max_isi, 10usize * max_isi as usize);

    // --------------------------------------------------------------------------
    // 2. compute the histogram and filter it
    let hist_values = hist(isi[..].as_ref(), ranges[..].as_ref())
        .iter()
        .map(|x| *x as f32 / isi_len as f32)
        .collect::<Vec<f32>>();
    let hist_norm = super::operations::math::lowess(hist_values[..].as_ref(), 0.05);

    // --------------------------------------------------------------------------
    // 3. get peaks
    // NOTE here i use the default parameters, maybe in the future some form of
    // LogIsiDescriptor struct will be required as function parameter
    let found_peaks = logisi_get_peaks(hist_norm[..].as_ref(), 2, 0f32, None);

    // --------------------------------------------------------------------------
    // 4. find the index of the intra burst peak

    // default value for the ISI threshold
    const ISI_THRESHOLD: f32 = 100f32; // milliseconds

    // find the max peak before the ISI_THRESHOLD
    let mut intra_index: isize = -1;
    let mut intra_value = f32::MIN;
    let mut intra_range = -1f32;
    let mut last_peak_index: isize = -1;

    for i in 0..found_peaks.0.len() {
        let index = found_peaks.0[i];
        let value = found_peaks.1[i];
        let range = ranges[index];

        if range < ISI_THRESHOLD {
            if value > intra_value {
                intra_index = index as isize;
                intra_value = value;
                intra_range = range;
                last_peak_index = i as isize;
            }
        } else {
            break;
        }
    }

    // 5. find the index of the first minimum after the threshold that maximize the void parameter,
    //    calculated as follow:
    //
    // void_parameter_i = 1 - h(min)/sqrt(h(max_pre)*h(max_post_i))
    // where max_post_i is the ith peak after the ISI threshold

    let void_parameter =
        move |min_val: f32, peak_2_val: f32| 1f32 - (min_val) / (intra_value * peak_2_val).sqrt();

    // get the index of the remaining peaks in the found_peaks array and check if the intra_peak
    // wasn't the last peak;
    if last_peak_index == found_peaks.0.len() as isize - 1 {
        return Err(SpikeError::LogISICalcThresholdIntraAtEndOfPeaks);
    }

    // advance to the next peak
    last_peak_index += 1;

    // allocating resources for storing the minima found as a array of
    // (min_index, min_value, void_parameter)
    let mut found_mins = vec![(0usize, 0f32, 0f32); found_peaks.0.len() - last_peak_index as usize]; // vector to hold the found minima respect to each
    let mut min_index = 0;
    let mut current_minima = f32::MAX;

    // here it cycle all the peaks from the next one after the intra burst peak, looks if it's a
    // minimum and assign it to the corrisponding maximum. Also check when a peak has been overcome
    // and updates the index of the peaks array
    // TODO check if it just returns the FIRST found minimum that overcome the void_threshold
    for i in intra_index as usize + 1..found_peaks.0.len() {
        let value = hist_values[i];
        let range = ranges[i];

        // check if the current value is a minima
        if value < current_minima {
            current_minima = value;
        }

        // MAYBE TODO check if a valid minimum has been found and break the cycle
        // check if the the current index is one a the current peak and in such case
        // update the corresponding *found_mins* value
        if i == found_peaks.0[last_peak_index as usize] {
            found_mins[min_index] = (
                i,
                current_minima,
                void_parameter(current_minima, found_peaks.1[last_peak_index as usize]),
            );
            last_peak_index += 1;
        }
    }

    // --------------------------------------------------------------------------
    // 6. return threshold
    //
    // return the first range of the histogram whose minimum overcome the void threshold

    const VOID_THRESHOLD: f32 = 0.7;

    return found_mins
        .iter()
        .filter(|min| min.2 > VOID_THRESHOLD)
        .next()
        .map(|min| Ok(ranges[min.0]))
        .unwrap_or(Err(
            SpikeError::LogISICalcThresholdNoMinWithRequiredVoidParameter,
        ));
}

// TODO handle the default value of cutoff to be 0.1
pub fn logisi_method(
    peak_train: &[usize],
    sampling_frequency: f32,
    cutoff: f32,
) -> Result<(), SpikeError> {
    if peak_train.len() <= 3 {
        return Err(SpikeError::LogISITooFewSamples);
    }

    let isi_threshold = logisi_calc_threshold(peak_train, sampling_frequency);

    todo!()
}
