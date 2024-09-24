from pathlib import Path
from os import listdir
from pycode.handlers.phaseh5 import PhaseH5

# SET THIS PARAMETERS TO THE REQUIERD VALUES
MULTIPLIER = 8  # the number of standard deviation of the signal to use as a threshold
PEAK_DURATION = 2e-3
REFRACTARY_TIME = 2e-3


# SET THE PATH OF THE FOLDER CONTAINING THE DATA CONVERTED WITH MULTICHANNEL DATA MANAGER
folder_path = "/home/leonardo/Documents/unige/data/18-07-2024/38894/raw/"

# Convert it to a Path object
folder_path = Path(folder_path)

for file in listdir(folder_path):
    if file.endswith(".h5"):
        phase_path = folder_path.joinpath(file)
        phase = PhaseH5(f"{phase_path}")
        phase.compute_all_peak_trains(MULTIPLIER, PEAK_DURATION, REFRACTARY_TIME)
