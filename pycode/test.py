import pycode as pc
import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path

filename = Path('e:/rust/spike-rs/test2.h5')
phase = pc.PyPhase(filename)

intervals = phase.get_digital_intervals(1)
bin_size = phase.sampling_frequency * 0.10 #s
channel_histos = phase.get_subsampled_pre_stim_post_from_intervals(intervals, int(bin_size))

max_pre = 0
max_stim = 0
max_post = 0

for channel, intervals in channel_histos.items():
    for data in intervals:
        if len(data[0]) > max_pre:
            max_pre = len(data[0])
        if len(data[1]) > max_stim:
            max_stim = len(data[1])
        if len(data[2]) > max_post:
            max_post = len(data[2])

print(max_pre)
print(max_stim)
print(max_post)
