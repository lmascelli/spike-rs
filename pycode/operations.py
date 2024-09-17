from typing import List, Optional, Tuple
import pycode_rs as pc


def compute_threshold(
    data: List[float], sampling_frequency: float, multiplier: float
) -> Optional[float]:
    return pc.compute_threshold(data, sampling_frequency, multiplier)


def spike_detection(
    data: List[float],
    sampling_frequency: float,
    threshold: float,
    peak_duration: float,
    refractory_time: float,
) -> Optional[Tuple[List[int], List[float]]]:
    return pc.spike_detection(
        data, sampling_frequency, threshold, peak_duration, refractory_time
    )


def subsample_peak_trains(phase, bin_size: int, digital_index: int):
    return pc.subsample_peak_trains(phase._phase, bin_size, digital_index)


def subsampled_post_stimulus_times(
    phase, bin_size: int, n_bins_post_stim: int, digital_index: int
):
    return pc.subsampled_post_stimulus_times(
        phase._phase, bin_size, int(n_bins_post_stim), digital_index
    )

def get_digital_intervals(digital: List[int]) -> List[Tuple[int, int]]:
    return pc.get_digital_intervals(digital)

def subsample_range(
    peaks: List[int], starting_sample: int, bin_size: int, n_bins: int
) -> List[int]:
    return pc.subsample_range(peaks, starting_sample, bin_size, n_bins)
