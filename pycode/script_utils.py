from pycode.handlers.phaseh5 import PhaseH5
import numpy as np

ERROR = -1
BASAL = 0
STIMULATION = 1
ELECTRIC_STIMULATION = 2

def guess_phase(phase: PhaseH5) -> int:
    n_digitals = phase.n_digitals()
    n_events = phase.n_events()
    
    if n_digitals == 0 and n_events > 0:
        return ELECTRIC_STIMULATION
    
    if n_digitals == 1 and n_events == 0:
        digital = np.array(phase.digital(0))
        if np.sum(digital) > 0:
            return STIMULATION
        else:
            return BASAL
    
    if n_digitals > 1:
        return ERROR

    return BASAL
