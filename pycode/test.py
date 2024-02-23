import pycode as pc
import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path

filename = Path('e:/rust/spike-rs/test2.h5')
phase = pc.PyPhase(filename)

intervals = phase.get_digital_intervals(1)
bin_size = phase.sampling_frequency * 0.010 #s
channel_histos = phase.get_subsampled_pre_stim_post_from_intervals(intervals, int(bin_size))

n_intervals = 0
max_pre = 0
max_stim = 0
max_post = 0

for channel, intervals in channel_histos.items():
    n_intervals = len(intervals)
    for data in intervals:
        if len(data[0]) > max_pre:
            max_pre = len(data[0])
        if len(data[1]) > max_stim:
            max_stim = len(data[1])
        if len(data[2]) > max_post:
            max_post = len(data[2])

pre = []
stim = []
post = []
tot = []

for n in range(n_intervals):
    this_pre = []
    this_stim = []
    this_post = []
    this_tot = []
    for i in range(max_pre):
        this_pre.append(0)
    for i in range(max_stim):
        this_stim.append(0)
    for i in range(max_post):
        this_post.append(0)
    for i in range(max_pre+max_stim+max_post):
        this_tot.append(0)
    pre.append(this_pre)
    stim.append(this_stim)
    post.append(this_post)
    tot.append(this_tot)

for channel, intervals in channel_histos.items():
    for i, data in enumerate(intervals):
        for j, val in enumerate(data[0]):
            pre[i][j] += val
            tot[i][j] += val
        for j, val in enumerate(data[1]):
            stim[i][j] += val
            tot[i][j+max_pre] += val
        for j, val in enumerate(data[2]):
            post[i][j] += val
            tot[i][j+max_pre+max_stim] += val

tot_np = np.array(tot)
plt.imshow(tot_np)
plt.plot([max_pre, max_pre], [-1, n_intervals + 1], 'r', linewidth = 2)
plt.plot([max_pre + max_stim, max_pre + max_stim], [-1, n_intervals + 1], 'r', linewidth = 2)
plt.show()
