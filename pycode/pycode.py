from typing import Dict, List, Tuple
from pathlib import Path
import pycode_rs as sp

class PyPhase:
    def __init__(self, filepath: Path):
        self._phase = sp.load_phase(str(filepath.absolute()))
        if self._phase is not None:
            self._valid = True
        else:
            self._valid = False

    def save(self, filepath: Path):
        if self._valid:
            sp.save(self._phase, str(filepath.absolute()))

    def peak_detection(self, peak_duration: float, refractary_time: float, n_devs: float):
        if self._valid:
            self._phase.compute_all_peak_trains(peak_duration, refractary_time, n_devs)

    def get_peaks_in_consecutive_intervals(self, intervals: List[Tuple[int, int]]
                                           ) -> Dict[str, List[int]]:
        return self._phase.get_peaks_in_consecutive_intervals(intervals)
