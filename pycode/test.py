import pycode as pc
import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path

filename = Path('e:/rust/spike-rs/test2.h5')
phase = pc.PyPhase(filename)
phase.clear_peaks_over_threshold(7e-5)
bin_size = phase.sampling_frequency * 0.020 #s

histos = phase.psth(bin_size, 1)
if histos is not None:
    img = np.array(histos)
    plt.imshow(img)
    plt.show()

# digital = phase.get_digital(1)
# intervals = phase.get_digital_intervals(1)
# 
# def check_intervals():
#     plt.plot(digital)
#     for interval in intervals:
#         plt.plot([interval[0], interval[0]], [0, 1])
#         plt.plot([interval[1], interval[1]], [0, 1])
# 
# 
# channel_histos = phase.get_subsampled_pre_stim_post_from_intervals(intervals, int(bin_size))
# 
# n_intervals = 0
# max_pre = 0
# max_stim = 0
# max_post = 0
# 
# for channel, intervals in channel_histos.items():
#     n_intervals = len(intervals)
#     for data in intervals:
#         if len(data[0]) > max_pre:
#             max_pre = len(data[0])
#         if len(data[1]) > max_stim:
#             max_stim = len(data[1])
#         if len(data[2]) > max_post:
#             max_post = len(data[2])
# 
# pre = []
# stim = []
# post = []
# tot = []
# 
# for n in range(n_intervals):
#     this_pre = []
#     this_stim = []
#     this_post = []
#     this_tot = []
#     for i in range(max_pre):
#         this_pre.append(0)
#     for i in range(max_stim):
#         this_stim.append(0)
#     for i in range(max_post):
#         this_post.append(0)
#     for i in range(max_pre+max_stim+max_post):
#         this_tot.append(0)
#     pre.append(this_pre)
#     stim.append(this_stim)
#     post.append(this_post)
#     tot.append(this_tot)
# 
# for channel, intervals in channel_histos.items():
#     for i, data in enumerate(intervals):
#         # print(data[0])
#         # print(data[2])
#         for j, val in enumerate(data[0]):
#             pre[i][j] += val
#             tot[i][j] += val
#         for j, val in enumerate(data[1]):
#             stim[i][j] += val
#             tot[i][j+max_pre] += val
#         for j, val in enumerate(data[2]):
#             post[i][j] += val
#             tot[i][j+max_pre+max_stim] += val
# 
# tot_sum = []
# for i in range(len(tot[0])):
#     tot_sum.append(0)
# 
# for row in tot:
#     for i, val in enumerate(row):
#         tot_sum[i] += val
# 
# fig, ax = plt.subplots(2)
# tot_np = np.array(tot)
# ax[0].bar(np.linspace(0, len(tot_np[0]), len(tot_np[0])), tot_sum)
# ax[1].imshow(tot_np)
# ax[1].plot([max_pre, max_pre], [-1, n_intervals + 1], 'r', linewidth = 2)
# ax[1].plot([max_pre + max_stim, max_pre + max_stim], [-1, n_intervals + 1], 'r', linewidth = 2)
# plt.show()


