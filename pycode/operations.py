from typing import List, Optional, Tuple
import numpy as np
from .pycode import (
    PyPhase,
    compute_threshold as py_compute_threshold,
    spike_detection as py_spike_detection,
    get_digital_intervals as py_get_digital_intervals,
    subsample_range as py_subsample_range
)

# TODO add those functions

def compute_threshold(
    data: List[float], sampling_frequency: float, multiplier: float
) -> Optional[float]:
    return py_compute_threshold(data, sampling_frequency, multiplier)


def spike_detection(
    data: List[float],
    sampling_frequency: float,
    threshold: float,
    peak_duration: float,
    refractory_time: float,
) -> Optional[Tuple[List[int], List[float]]]:
    return py_spike_detection(
        data, sampling_frequency, threshold, peak_duration, refractory_time
    )

def get_digital_intervals(digital: List[int]) -> List[Tuple[int, int]]:
    return py_get_digital_intervals(digital)


def subsample_range(
    peaks: List[int], starting_sample: int, bin_size: int, n_bins: int
) -> List[int]:
    return py_subsample_range(peaks, starting_sample, bin_size, n_bins)


def psth(phase: PyPhase, bin_time_duration: float, psth_duration: float) -> List[int] | np.ndarray:
    """
    Compute the PSTH ociaoooooooo :):):)
    and returns a list with the count of the spikes in each bin.

    @Parameters
    - phase: the Phase of interest
    - bin_time_duration: the duration of the bin IN SECONDS
    - psth_duration: the duration of the whole psth IN SECONDS
    """
    # OPEN THE PYCODE_RS HANDLER FOR THE DATA
    sampling_frequency = phase.sampling_frequency()
    bin_size = int(
        sampling_frequency * bin_time_duration
    )  # this round the size of a bin to the lower integer
    n_bins = int(psth_duration / bin_time_duration)  # number of bin after the stimulus

    channels = phase.labels()  # list of all the available channels

    # get the number of digital channels. if it's different from 1 an error has occurred
    # during the recording phase
    n_digital = phase.n_digitals()
    if n_digital != 1:
        exit(f"ERROR: the stimulation phase has {n_digital} digital channels")

    res = [0] * n_bins  # variable to accumulate the psth

    # read the digital channel
    digital = phase.digital(0)
    # get the interval timestamps where the stimulation is active
    digital_intervals = get_digital_intervals(digital)

    for interval in digital_intervals:
        for channel in channels:
            res = np.add(
                res,
                subsample_range(
                    phase.peak_train(channel, None, None)[0],
                    interval[0],
                    bin_size,
                    n_bins,
                ),
            )

    return res
