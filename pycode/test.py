import pycode as pc
from mc_explorer import MCExplorer

from pathlib import Path

BASAL = 0
DIGIAL = 1
STIM = 2

base_folder = Path("/home/leonardo/Documents/unige/raw data/12-04-2024/")
batch_folder= "38936_DIV77/"  
filename = "03_basal.h5"
explorer = MCExplorer(base_folder.joinpath(batch_folder).joinpath(filename))
print(explorer)
convert = BASAL
if convert == BASAL:
    phase = explorer.convert_phase(0, 0, None, None)
    # phase = explorer.convert_phase(0, 0, 1, None)
    phase.save(base_folder.joinpath(batch_folder).joinpath('hdf5').joinpath(filename))
