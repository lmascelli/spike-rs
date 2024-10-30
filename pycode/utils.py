from typing import Optional
from .pycode import PyPhase
from . import settings
import matplotlib.pyplot as plt
import numpy as np

def plot_raw_with_spikes(phase: PyPhase,
                         label: str,
                         start: Optional[int] = None,
                         end: Optional[int] = None,
                         ):
    raw_data = phase.raw_data(label, start, end)
    peaks = phase.peak_train(label, start, end)

    x_start = 0
    if start is not None:
        x_start = start
    x_end = phase.datalen()
    if end is not None:
        x_end = end

    times = np.arange(x_start, x_end - 1, 1)

    plt.plot(times, raw_data, color = settings.RAW_DATA_COLOR)
    plt.stem(peaks[0], peaks[1],
             linefmt=f"{settings.SPIKES_COLOR_LINE}",
             markerfmt=f"{settings.SPIKES_COLOR_HEAD}")
