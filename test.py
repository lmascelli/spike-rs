import spike_rs as sp
import matplotlib.pyplot as plt

filename = 'e:/rust/spike-rs/test.h5'
a = sp.load_phase(filename)
for label in a.channel_labels:
    print(label, a.spikes_count(label))
