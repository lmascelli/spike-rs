from typing import List, Optional, Tuple
import numpy as np
from statsmodels.nonparametric.smoothers_lowess import lowess

from .pycode import (
    PyPhase,
    compute_threshold as py_compute_threshold,
    spike_detection as py_spike_detection,
    get_digital_intervals as py_get_digital_intervals,
    subsample_range as py_subsample_range,
)


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


def psth(phase: PyPhase, bin_time_duration: float, psth_duration: float) -> np.ndarray:
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
        exit(
            f"ERROR: the stimulation phase has {n_digital} digital channels (grazie MultiChannel)"
        )

    res = np.zeros(n_bins)  # variable to accumulate the psth

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

    return res / (len(channels) * len(digital_intervals))


# paremeters for hist:
# minimum peaks distances (2 bins default)
# ISI th (100 ms default)
# void threshold (0.7 default)


def ISI_hist_log(
    peak_times: List[int],
    duration: int,
    sampling_frequency: float,
    n_bins_per_decade: int = 10,
) -> Optional[Tuple[List[float], np.ndarray]]:
    """
    Compute the normalized ISI histogram of the given spikes
    """
    if len(peak_times) < 2:
        return None
    else:
        # Threshold of MFR for considering a channel dead
        MFR_THRESHOLD = 0.1
        mfr = float(len(peak_times)) / float(duration / sampling_frequency)
        if mfr < MFR_THRESHOLD:
            return None
        allISI = np.diff(peak_times) * 1000 / (sampling_frequency)  # in seconds
        max_win = int(np.ceil(np.log10(np.max(allISI))))
        print(max_win)
        bins = np.logspace(0, max_win, max_win * n_bins_per_decade)
        hist_values, hist_bins = np.histogram(allISI, bins=bins)
        hist_norm = hist_values / np.sum(hist_values)
        SPAN = 5
        # do the lowess regression method
        hist_smoothed = np.convolve(hist_norm, np.ones(SPAN) / SPAN, mode="same")
        return (hist_smoothed, hist_bins[:-1])


def ISI_hist_log_all_channels(
    phase: PyPhase,
) -> Optional[Tuple[List[float], List[int]]]:
    """ """
    voidParamTh = 0.7  # non so cosa sia
    ISITh = 100  # ms valore di default se non si trova la soglia

    ISImax = np.zeros(len(phase.labels()))
    flags = np.zeros(shape=(len(phase.labels()), 2))
    pks = []

    for label in phase.labels():
        hist, bins = ISI_hist_log(
            phase.peak_train(label), phase.datalen(), phase.sampling_frequency()
        )
        if len(bins) > 0:
            pass
    return None


def logisi_get_peaks(hist: np.ndarray, Pd: float = 2, Th: float = 0, Np = None):
    """
    Finds peaks in logISI histogram
    """

    m = 0
    L = len(hist)
    j = 0
    if Np is None:
        Np = L
    pks = []
    locs = []
    void_threshold = 0.7
    while j < L and m < Np:
        j = j + 1
        endL = np.max([1, j - Pd])
        if m > 0 and j < np.min([locs[m] + Pd, L-1]):
            j = np.min([locs[m] + Pd, L-1])
            endL = j - Pd
        endR = np.min([L, j + Pd])
    temp = hist[endL:endR]


def logisi_find_thresh(hist: np.ndarray, ISITh: float = 100):
    void_threshold = 0.7
    get_peaks = logisi_get_peaks(hist)


def logisi_break_calc(spike_train: List[int], cutoff):
    """
    Calculate the cufoff for burst detection.

    # Parameters:
    - spike_train: the list of times where a spike has been detected (IN SECONDS)
    - cutoff:

    # Returns:
    the cutoff for burst detection
    """

    isi = (
        np.diff(spike_train) * 1000
    )  # compute the isi between the spikes and convert it in milliseconds
    max_isi = int(np.ceil(np.log10(np.max(isi))))
    isi = isi[isi >= 1]
    breakpoints = np.logspace(0, max_isi, 10 * max_isi)
    hist, edges = np.histogram(isi, bins=breakpoints)
    norm_hist = hist / np.sum(hist)
    norm_hist = lowess(norm_hist, np.arange(len(norm_hist)), frac=0.05)[:, 1]
    threshold = logisi_find_thresh(norm_hist, cutoff * 1000)
    if threshold is not None:
        return threshold / 1000


def logisi_method(spike_train: List[int], cutoff: float = 0.1) -> Optional[float]:
    if len(spike_train) > 3:
        # Calculate threshold as iso_low
        _isi_low = logisi_break_calc(spike_train, cutoff)
        pass
    else:
        return None
