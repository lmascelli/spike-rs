from mc_explorer import MCExplorer
import pycode as pc
from pathlib import Path
import matplotlib.pyplot as plt
import numpy as np

convert_filename = Path("/home/leonardo/Documents/unige/raw data/raw_test.h5")

# TEST H5Content
content = MCExplorer(convert_filename)
print(content.list_recordings())
print(content.list_analogs(0))
print(content.list_analog_channels(0, 0))
data = content.get_channel_data(0, 0, 'E-00155 83')

plt.plot(data)
plt.show()
