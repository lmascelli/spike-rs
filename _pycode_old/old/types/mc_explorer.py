from pathlib import Path
from typing import Callable, List, Optional, Tuple

from .. import pycode_rs as sp  # type: ignore
from ..scripts.converting_rules import ConvertingValues
from ..types.pyphase import PyPhase

################################################################################
#                             MCExplorer
################################################################################

class MCExplorer:
    def __init__(self, filename: Path):
        self.filename = filename.absolute()
        self.explorer = sp.MCExplorer(str(filename.absolute()))

    def list_recordings(self) -> Optional[List[Tuple[int, str]]]:
        return self.explorer.list_recordings()

    def recording_info(self, recording_index: int) -> Optional[str]:
        return self.explorer.recording_info(recording_index)

    def list_analogs(self, recording_index: int) -> Optional[List[Tuple[int, str]]]:
        return self.explorer.list_analogs(recording_index)

    def analog_info(self, recording_index: int, analog_index: int) -> Optional[str]:
        return self.explorer.analog_info(recording_index, analog_index)

    def analog_dims(self, recording_index: int, analog_index: int) -> Optional[List[int]]:
        return self.explorer.analog_dims(recording_index, analog_index)

    def list_analog_channels(self, recording_index: int, analog_index: int
                             ) -> Optional[List[str]]:
        return self.explorer.list_analog_channels(recording_index, analog_index)

    def get_channel_data(self, recording_index: int,
                         analog_index: int,
                         channel_label: str) -> Optional[List[float]]:
        return self.explorer.get_channel_data(recording_index, analog_index, channel_label)

    def convert_phase(self, recording_index: int,
                      raw_data_index: int,
                      digital_index: Optional[int],
                      event_index: Optional[int]) -> Optional[PyPhase]:
        return PyPhase(self.explorer.convert_phase(recording_index,
                                                   raw_data_index,
                                                   digital_index,
                                                   event_index))

    def convert_with_rule(self,
                          rule: Callable[[str], Optional[ConvertingValues]],
                          dest: str):
        converting_values = rule(self.filename.name)
        cond = "basal"
        digital_index = None
        event_index = None
        if converting_values.cond.upper().find('STIMEL') >= 0:
            cond = "stimel"
            event_index = 0
        elif converting_values.cond.upper().find('STIM') >= 0:
            cond = "digital"
            digital_index = 1

        phase = self.convert_phase(0, 0, digital_index, event_index)

        phase.save_as_mat(converting_values.matrice,
                         converting_values.cond,
                         converting_values.div,
                         converting_values.i,
                         converting_values.t,
                         dest)

    def __str__(self) -> str:
        if self.explorer is not None:
            return f"{self.explorer}"
        else:
            return "No content loaded"
