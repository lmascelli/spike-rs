from pycode.handlers.phaseh5 import PhaseH5

phase = PhaseH5(
    "/home/leonardo/Documents/unige/data/18-07-2024/38927/raw/2024-07-18T15-49-0838927_100E_DIV70_nbasal_0004_E-00155.h5"
)

print(phase.compute_all_peak_trains(8, 2e-3, 2e-3))
