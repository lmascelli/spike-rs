from typing import List, Optional, Tuple

import pycode_rs as sp
from pathlib import Path

################################################################################
#                             MCExplorer
################################################################################

class MCExplorer:
    def __init__(self, filename: Path):
        self.explorer = sp.MCExplorer(str(filename.absolute()))

    def list_recordings(self) -> Optional[List[Tuple[int, str]]]:
        return self.explorer.list_recordings()

    def recording_info(self, recording_index: int) -> Optional[str]:
        return self.explorer.recording_info(recording_index)

    def list_analogs(self, recording_index: int) -> Optional[List[Tuple[int, str]]]:
        return self.explorer.list_analogs(recording_index)

    def analog_info(self, recording_index: int, analog_index: int) -> Optional[str]:
        return self.explorer.analog_info(recording_index, analog_index)

    def list_analog_channels(self, recording_index: int, analog_index: int
                             ) -> Optional[List[str]]:
        return self.explorer.list_analog_channels(recording_index, analog_index)

    def get_channel_data(self, recording_index: int,
                         analog_index: int,
                         channel_label: str) -> Optional[List[float]]:
        return self.explorer.get_channel_data(recording_index, analog_index, channel_label)
