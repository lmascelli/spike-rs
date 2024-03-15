import pycode as pc
from pathlib import Path
import matplotlib.pyplot as plt
import numpy as np

# convert_filename = Path("f:/03-11-2023/34341/hdf5/2024-03-14T16-46-3738891_100E_DIV49_StimUS65_0007_E-00155.h5")
phase_filename = Path("test.h5")
# convert_res = pc.convert_mc_h5_phase(convert_filename, phase_filename)
phase = pc.PyPhase(phase_filename)
print(phase.channel_labels)
