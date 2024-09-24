from typing import List, Optional, Tuple
from ..pycode_rs import PyPhaseH5
from ..operations import compute_threshold, spike_detection


class PhaseH5:
    def __init__(self, filename: str):
        self._phase = PyPhaseH5(filename)

    def datalen(self) -> int:
        return self._phase.datalen()

    def labels(self) -> List[str]:
        return self._phase.labels()

    def sampling_frequency(self) -> float:
        return self._phase.sampling_frequency()

    def raw_data(
        self, channel: str, start: Optional[int] = None, end: Optional[int] = None
    ) -> List[float]:
        return self._phase.raw_data(channel, start, end)

    def set_raw_data(
        self,
        channel: str,
        data: List[float],
        start: Optional[int] = None,
    ) -> bool:
        return self._phase.set_raw_data(channel, start, data) is not None

    def n_digitals(self) -> int:
        return self._phase.n_digitals()

    def digital(
        self, index: int, start: Optional[int] = None, end: Optional[int] = None
    ) -> List[float]:
        return self._phase.digital(index, start, end)

    def set_digital(
        self,
        index: int,
        data: List[float],
        start: Optional[int] = None,
    ) -> bool:
        return self._phase.set_digital(index, start, data) is not None

    def n_events(self) -> Optional[int]:
        return self._phase.n_events()

    def events(self, index: int) -> Optional[List[int]]:
        return self._phase.events(index)

    def peak_train(
        self, channel: str,
        start: Optional[int] = None,
        end: Optional[int] = None
    ) -> Optional[Tuple[List[int], List[float]]]:
        return self._phase.peak_train(channel, start, end)

    def set_peak_train(
        self,
        channel: str,
        data: Tuple[List[int], List[float]],
        start: Optional[int] = None,
        end: Optional[int] = None,
    ) -> bool:
        return self._phase.set_peak_train(channel, data, start, end)

    def compute_all_peak_trains(
        self, multiplier: float, peak_duration: float, refractory_time: float
    ) -> bool:
        """
        Compute all the peak trains for all the channels of this phase.
        Return True if the operation succeded, False otherwise
        """

        labels = self.labels()
        for i, label in enumerate(labels):
            print(f"{i}/{len(labels)} Computing peak trains for channel: {label}")
            data = self.raw_data(label)
            sf = self.sampling_frequency()
            this_result = self.set_peak_train(
                label,
                spike_detection(
                    data,
                    sf,
                    compute_threshold(data, sf, multiplier),
                    peak_duration,
                    refractory_time,
                ),
            )
            if this_result is False:
                print(f"Failed to compute the peak trains for channel {label}")
                return False

        return True
