use std::collections::HashMap;
use crate::{
    operations::math,
    error::SpikeError,
    types::PhaseHandler,
    analysis::subsampling::subsample_range,
};

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

pub mod logisi {
    /// Finds peaks in an histogram representing the density distriburion of ISI of a
    /// peak train.
    pub fn get_peaks(
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
    pub fn calc_threshold(
        peak_train: &[usize],
        sampling_frequency: f32,
    ) -> Result<f32, super::SpikeError> {
        // --------------------------------------------------------------------------
        // 1. get the differences in number of samples between adjacent peaks and
        // convert them in milliseconds
        let isi: Vec<f32> = super::math::diff(peak_train)
            .iter()
            .map(|x| *x as f32 / sampling_frequency as f32 * 1000f32)
            .collect();

        let isi_len = isi.len();

        let max_isi = isi
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less))
            .expect("calc_threshold: Failed to find the ISI max")
            .clone()
            .ceil();

        let ranges = super::math::logspace(0f32, max_isi, 10usize * max_isi as usize);

        // --------------------------------------------------------------------------
        // 2. compute the histogram and filter it
        let hist_values = hist(isi[..].as_ref(), ranges[..].as_ref())
            .iter()
            .map(|x| *x as f32 / isi_len as f32)
            .collect::<Vec<f32>>();
        let hist_norm = super::math::lowess(hist_values[..].as_ref(), 0.05);

        // --------------------------------------------------------------------------
        // 3. get peaks
        // NOTE here i use the default parameters, maybe in the future some form of
        // LogIsiDescriptor struct will be required as function parameter
        let found_peaks = get_peaks(hist_norm[..].as_ref(), 2, 0f32, None);

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

        let void_parameter = move |min_val: f32, peak_2_val: f32| {
            1f32 - (min_val) / (intra_value * peak_2_val).sqrt()
        };

        // get the index of the remaining peaks in the found_peaks array and check if the intra_peak
        // wasn't the last peak;
        if last_peak_index == found_peaks.0.len() as isize - 1 {
            return Err(super::SpikeError::LogISICalcThresholdIntraAtEndOfPeaks);
        }

        // advance to the next peak
        last_peak_index += 1;

        // allocating resources for storing the minima found as a array of
        // (min_index, min_value, void_parameter)
        let mut found_mins =
            vec![(0usize, 0f32, 0f32); found_peaks.0.len() - last_peak_index as usize]; // vector to hold the found minima respect to each
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
            .map(|min| Ok(ranges[min.0] / 1000.))
            .unwrap_or(Err(
                super::SpikeError::LogISICalcThresholdNoMinWithRequiredVoidParameter,
            ));
    }

    pub fn find_burst(
        peak_train: &[usize],
        min_ibi: f32,
        min_durn: f32,
        min_spikes: usize,
        isi_low: f32,
    ) -> Result<(), super::SpikeError> {
        // Create a temp array for the storage of the bursts.  Assume that
        // it will not be longer than Nspikes/2 since we need at least two
        // spikes to be in a burst.

        let max_burst = peak_train.len() / 2;
        let mut burst_start = vec![0usize; max_burst];
        let mut burst_end = vec![0usize; max_burst];
        let mut burst_isi = vec![0usize; max_burst];

        // 1. Each interspike interval of the data is compared with the threshold
        // THRE. If the interval is greater than the threshold value, it can not be
        // part of a burst; if the interval is smaller or equal to the threhold, the
        // interval may be part of a burst.

        const EPSILON: f32 = 1e-10;
        let mut in_burst = false;
        let mut n = 2;

        let mut beg = n;
        let mut end = n - 1;

        let mut last_end = None;
        let mut ibi = 0;

        while n < peak_train.len() {
            // TODO(performace increase) avoid recomputing the isi using directly the
            // diff calculated during previous evaluations
            let next_isi = (peak_train[n] - peak_train[n - 1]) as f32;

            if in_burst {
                if next_isi > isi_low + EPSILON {
                    // then we are no more in a burst
                    in_burst = false;
                    end = n - 1; // the end of the burst is the spike before

                    // if an other burst has been found before calculate the IBI
                    // and update the last burst time
                    if last_end.is_some() {
                        ibi = peak_train[beg] - last_end.unwrap();
                    }
                    last_end.replace(peak_train[end]);

                    // add the burst to the return lists
                    burst_start.push(beg);
                    burst_end.push(end);
                    burst_isi.push(ibi);
                    if burst_start.len() > max_burst {
                        return Err(super::SpikeError::LogISIFindBurstTooManyBursts);
                    }
                }
            } else {
                if next_isi < isi_low + EPSILON {
                    // then we are at the start of a burst
                    in_burst = true;
                    beg = n - 1; // the start of the burst is the spike before
                }
            }
        }

        Ok(())
    }

    // TODO handle the default value of cutoff to be 0.1
    pub fn method(
        peak_train: &[usize],
        sampling_frequency: f32,
        cutoff: f32,
    ) -> Result<(), super::SpikeError> {
        if peak_train.len() <= 3 {
            return Err(super::SpikeError::LogISITooFewSamples);
        }

        // find burst parameters
        let min_ibi;
        let min_durn = 0f32;
        let min_spikes = 3;
        let isi_low;

        // calculate the isi threshold and consequentely set the find burst parameters
        let isi_threshold = calc_threshold(peak_train, sampling_frequency)?;
        // Cases to handle:
        // - log_threshold > 1
        // - cutoff < log_threshold < 1
        // - log_threshold < 0 (i'm not sure if it's necessary to handle this)
        // - other cases
        if isi_threshold > 1f32 {
            min_ibi = 0f32;
            isi_low = cutoff;
        } else if isi_threshold >= cutoff && isi_threshold <= 1f32 {
            min_ibi = isi_threshold;
            isi_low = cutoff;
        } else {
            min_ibi = 0f32;
            isi_low = isi_threshold;
        }

        find_burst(peak_train, min_ibi, min_durn, min_spikes, isi_low)
    }
}
