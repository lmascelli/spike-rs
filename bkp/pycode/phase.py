from typing import List, Optional, Tuple
from pathlib import Path


class PyCodeFileNotFound(Exception):
    def __init__(self, filepath: str):
        "A file does not exists"
        self.message = f"{filepath} does not exists"
        super().__init__(filepath)


class Phase:
    """
    This is a wrapper of some kind of implementation of a Phase.
    This implementation (at the moment there is just one "native_c" but you
    can define others) should implement all the methods that this class
    wraps.
    """

    def __init__(self, handler, filepath: str | Path):
        "Open the file at path FILEPATH with the HANDLER implementation of Phase"

        # test the existence of the file at path
        if not Path(filepath).exists():
            raise PyCodeFileNotFound(f"{filepath}")

        # open the handler for that file
        self.handler = handler(filepath)

    def datalen(self) -> Optional[int]:
        "Returns the number of samples in this phase recording"
        return self.handler.get().datalen()

    def sampling_frequency(self) -> Optional[float]:
        "Returns the sampling frequency used during the recording"
        return self.handler.get().sampling_frequency()

    def labels(self) -> Optional[List[str]]:
        "Return a list of all the channel labels contained in the recording"
        return self.handler.get().labels()

    def raw_data(
        self, channel: str, start: Optional[int] = None, end: Optional[int] = None
    ) -> Optional[List[float]]:
        "Return the raw data between START and END"
        return self.handler.get().raw_data(channel, start, end)

    def set_raw_data(
        self, channel: str, data: List[float], start: Optional[int] = None
    ) -> Optional[bool]:
        "Set the raw data of CHANNEL to DATA starting from START. Returns True if the operation succeded"
        return self.handler.get().set_raw_data(channel, data, start)

    def n_digitals(self) -> Optional[usize]:
        "Returns the number of digital channels present in the recording"
        return self.handler.get().n_digital()

    def digital(
        self, index: int, start: Optional[int] = None, end: Optional[int] = None
    ) -> Optional[List[float]]:
        "Returns the digital channel data of the INDEX channel between START and END"
        return self.handler.get().digital(index, start, end)

    def set_digital(
        self,
        index: int,
        data: List[f32],
        start: Optional[int],
    ) -> Optional[bool]:
        "Set the digital data of the index channel to DATA starting from START. Returns True if the operation succeeded"
        return self.handler.get().set_digital(index, data, start)

    def n_events(self) -> Optional[int]:
        "Returns the number of event arrays contained in the recording"
        return self.handler.get().n_events()

    def events(self, index: int) -> Optional[List[int]]:
        "Returns the event array at INDEX"
        return self.handler.get().events(index)

    def peak_train(
        self, channel: str, start: Optional[int], end: Optional[int]
    ) -> Optional[Tuple[List[int], List[float]]]:
        "Returns the peaks sample index and amplitude of CHANNEL between START and END"
        return self.handler.get().peak_train(channel, start, end)

    def set_peak_train(
        self,
        channel: str,
        data: Tuple[List[int], List[float]],
        start: Optional[int],
        end: Optional[int],
    ) -> Optional[bool]:
        "Set the peaks sample index and amplitude of CHANNEL between START and END"
        return self.handler.get().set_peak_train(channel, data, start, end)
