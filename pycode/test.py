import pycode as pc
from pathlib import Path

convert_filename = Path("f:/03-11-2023/34341/hdf5/2024-03-06T17-06-25prova2 stim_E-00155.h5")
phase_filename = Path("test.h5")
convert_res = pc.convert_mc_h5_phase(convert_filename, phase_filename)
if convert_res:
    phase = pc.PyPhase(phase_filename)
    print(phase.get_el_stim_intervals())
