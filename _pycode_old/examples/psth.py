from pycode.operations import psth
from pycode.handlers.phaseh5 import PhaseH5
import matplotlib.pyplot as plt
import numpy as np


# SET THE PATH OF THE FILE CONTAINING THE DATA CONVERTED WITH MULTICHANNEL DATA MANAGER
phase_file = "/home/leonardo/Documents/unige/data/18-07-2024/38927/raw/2024-07-18T15-36-4438927_100E_DIV70_Stim70_0002_E-00155.h5"
phase = PhaseH5(phase_file)
bin_time_duration = 10e-3  # duration of a bin IN SECONDS
psth_duration = 1000e-3  # duration of the psth IN SECONDS

psth_vals = psth(phase, bin_time_duration, psth_duration)

plt.bar(np.linspace(0, len(psth_vals), len(psth_vals)), psth_vals)
plt.show()
