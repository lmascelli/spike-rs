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
