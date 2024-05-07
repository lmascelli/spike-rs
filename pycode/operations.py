from typing import List, Optional
import pycode.pycode_rs as pc
from pycode.types.pyphase import PyPhase

def psth(phase: PyPhase, bin_size: int, digital_index: int) -> Optional[List[List[int]]]:
    return pc.psth(phase._phase, bin_size, digital_index)
