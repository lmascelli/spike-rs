import spyke_rs as sp
import matplotlib.pyplot as plt

filename = 'e:/rust/spike-rs/test.h5'
a = sp.load_phase(filename)

raw_data = a.get_raw_data('46')
peaks_train = a.get_peaks_train('46')
plt.plot(raw_data)
plt.scatter(peaks_train[1], peaks_train[0], c='red')
plt.show()
