from typing import List
from ..pycode_rs import PyPhaseH5

class PhaseH5:
    def __init__(self, filename: str):
        self._phase = PyPhaseH5(filename)

    def datalen(self) -> int:
        self._phase.datalen()

    def labels(self) -> List[str]:
        self._phase.labels()

    def sampling_frequency(self) -> float:
        self._phase.sampling_frequency()
