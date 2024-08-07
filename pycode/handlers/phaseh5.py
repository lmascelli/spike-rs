from typing import List, Optional, Tuple
from ..pycode_rs import PyPhaseH5


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
        return self._phase.raw_data(index, start, end)

    def set_digital(
        self,
        index: int,
        data: List[float],
        start: Optional[int] = None,
    ) -> bool:
        return self._phase.set_raw_data(index, start, data) is not None

    def n_events(self) -> Optional[int]:
        return self._phase.n_events()

    def events(self, index: int) -> Optional[List[int]]:
        return self._phase.events(index)

    def peak_train(
        self, channel: str, start: Optional[int], end: Optional[int]
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
