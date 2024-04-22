import pycode as pc
from os import listdir

from pathlib import Path
import matplotlib.pyplot as plt

def plot_rasterplot(phase):
    electrodes = []
    spikes = []
    for label in phase.channel_labels:
        electrodes.append(label)
        spikes.append(phase.get_peaks_train(label)[1][:])
    plt.eventplot(spikes)
    plt.show()

base_path = Path("/home/leonardo/Documents/unige/raw data/done/39480_DIV77/")
for current_phase in listdir(base_path.joinpath('hdf5')):
    filename = base_path.joinpath("hdf5").joinpath(current_phase)
    savefile = base_path.joinpath("hdf5_peak").joinpath(current_phase)

    print(filename)

    phase = pc.PyPhase.from_file(filename)
    phase.peak_detection(2e-3, 2e-3, 8)
    phase.save(savefile)

    electrodes = []
    spikes = []
    for label in phase.channel_labels:
        electrodes.append(label)
        spikes.append(phase.get_peaks_train(label)[1][:])
    plt.eventplot(spikes)
    plt.savefig(base_path.joinpath('images').joinpath(f"{current_phase[:-3]}.jpg"))
