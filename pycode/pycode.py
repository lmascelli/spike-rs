from typing import Dict, List, Optional, Tuple
from pathlib import Path
import pycode_rs as sp

def convert_mc_h5_phase(source: Path, dest: Path) -> bool:
    """
    Convert an HDF5 file obtained from MultiChannel Data Manager into a PyClass
    readable file

    Args:
        source (Path): path of the source file
        dest (Path): path of the destination file

    Returns:
        None
    """
    result = sp.convert_mc_h5_file(str(source.absolute()), str(dest.absolute()))
    if result == 0:
        return True
    else:
        return False

def check_valid_bin_size(interval: Tuple[int, int], bin_size: int) -> int:
    return sp.check_valid_bin_size(interval, bin_size)

class PyPhase:
    """
    Contains and manage data for a phase recording

    Attributes:
        sampling_frequency (float):         the sampling frequency of the recordings
        channel_labels (List[str]):         a collection of all the active electrode labels
        digitals_lenghts (List[int]):       a collection of the lenghts of the digitals channels
        raw_data_lenghts (Dict[str, int])   a map between the raw data labels and their data lenghts
        peak_train_lenghts (Dict[str, int]) a map between the peak train labels and their data lenghts
    """
    def __init__(self, filepath: Path):
        """
        Load an HDF5 phase file and construct a PyPhase instance.
        The private attribute `_valid` will be set to True if the load succeed, to False otherwise
        """
        self._phase = sp.load_phase(str(filepath.absolute()))
        if self._phase is not None:
            self.digitals_lengths = self._phase.digitals_lengths
            self.sampling_frequency = self._phase.sampling_frequency
            self.raw_data_lengths = self._phase.raw_data_lengths
            self.peak_train_lengths = self._phase.peak_train_lengths
            self.channel_labels = self._phase.channel_labels
            self._valid = True
        else:
            self._valid = False

    def update(self):
        """
        Updates the fields of this instance"
        """
        self._phase.update()

    def save(self, filepath: Path):
        """
        Save this instance of PyPhase to the given filepath

        Args:
            filepath (Path): path of the save file
        """
        if self._valid:
            sp.save_phase(self._phase, str(filepath.absolute()))

    def get_digital(self, index: int) -> Optional[List[float]]:
        """
        Query for the `index`th digital signal

        Args:
            label (str)

        Returns:
            (Optional[List[Float]]): the array of the digital signal if found, None otherwise
        """

        return self._phase.get_digital(index)

    def get_raw_data(self, label) -> Optional[List[float]]:
        """
        Query for the raw data for the given label

        Args:
            label (str)

        Returns:
            (Optional[List[Float]]): the array of the raw data if found, None otherwise
        """
        return self._phase.get_raw_data(label)

    def get_peaks_train(self, label) -> Optional[List[int]]:
        """
        Query for the peaks train for the given label

        Args:
            label (str)

        Returns:
            (Optional[List[float]]): the list of the peaks if found, None otherwise
        """
        return self._phase.get_peaks_train(label)

    def get_peaks_bins(self, n_bins: int = 50) -> Dict[str, Tuple[List[int], float, float]]:
        """
        Build an histogram of the peaks magnitude distribution for each channel

        Args:
            n_bins (int): number of bins in the histogram

        Returns:
            (Dict[str, Tuple[List[int], float, float]]): a map between the channel name
                and the bin histogram, the min and the max values of the bins
        """
        return self._phase.get_peaks_bins(n_bins)

    def peak_detection(self, peak_duration: float, refractary_time: float, n_devs: float):
        """
        Computes the peak detection on this phase.

        Args:
            peak_duration (float)
            refractary_time (float)
            n_devs (float): multiplier for the automatical computed threshold
        """
        if self._valid:
            self._phase.compute_all_peak_trains(peak_duration, refractary_time, n_devs)

    def get_peaks_in_consecutive_intervals(self, intervals: List[Tuple[int, int]]
                                           ) -> Dict[str, List[List[int]]]:
        """
        Extract the peaks in the queried intervals

        Args:
            intervals: (List[Tuple[int, int]]) : list of the boundaries (start, end)
                                                 of the intervals

        Returns:
            (Dict[str, List[int]]): a map between the label of the channels and the
                                    list of the peaks founded in the queried intervals
        """
        return self._phase.get_peaks_in_consecutive_intervals(intervals)

    def get_digital_intervals(self, index) -> List[Tuple[int, int]]:
        """
        Extract the boundaries (start, end) of the choosen signal

        Args:
            index (int): the index of the digital signal

        Returns:
            (List[Tuple[int, int]]): a collection of the boundaries of the intevals
        """
        return self._phase.get_digital_intervals(index)

    def clear_peaks_over_threshold(self, threshold: float):
        """
        Removes from each peaks train all the peaks where the absolute value is
        greater than `threshold`

        Args:
            threshold (float)
        """
        self._phase.clear_peaks_over_threshold(threshold)

    def get_subsampled_pre_stim_post_from_intervals(self, intervals: List[Tuple[int, int]],
                                                    bin_size: int
                                                    ) -> Dict[str, List[Tuple[List[int], List[int], List[int]]]]:
        return self._phase.get_subsampled_pre_stim_post_from_intervals(intervals, int(bin_size))
