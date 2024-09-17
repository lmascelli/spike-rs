from pycode.handlers.phaseh5 import PhaseH5
from pycode.operations import get_digital_intervals, subsample_range
import matplotlib.pyplot as plt
import numpy as np


# SET THE PATH OF THE FILE CONTAINING THE DATA CONVERTED WITH MULTICHANNEL DATA MANAGER
phase_file = "/home/leonardo/Documents/unige/data/18-07-2024/38927/raw/2024-07-18T15-36-4438927_100E_DIV70_Stim70_0002_E-00155.h5"

# OPEN THE PYCODE_RS HANDLER FOR THE DATA
phase = PhaseH5(phase_file)

sampling_frequency = phase.sampling_frequency()
bin_time_duration = 10e-3  # duration of a bin IN SECONDS
bin_size = int(
    sampling_frequency * bin_time_duration
)  # this round the size of a bin to the lower integer
psth_duration = 500e-3  # duration of the psth IN SECONDS
n_bins = int(psth_duration / bin_time_duration)  # number of bin after the stimulus

channels = phase.labels()  # list of all the available channels

# get the number of digital channels. if it's different from 1 an error has occurred
# during the recording phase
n_digital = phase.n_digitals()
if n_digital != 1:
    exit(f"ERROR: the stimulation phase has {n_digital} digital channels")

res = [0] * n_bins  # variable to accumulate the psth

# read the digital channel
digital = phase.digital(0)
# get the interval timestamps where the stimulation is active
digital_intervals = get_digital_intervals(digital)

for interval in digital_intervals:
    for channel in channels:
        res = np.add(
            res,
            subsample_range(
                phase.peak_train(channel, None, None)[0], interval[0], bin_size, n_bins
            ),
        )

plt.bar(np.linspace(0, n_bins, n_bins), res)
plt.show()
