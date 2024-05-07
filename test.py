from os import getenv, listdir, mkdir
import sys

sys.path.insert(0, getenv("PYCODE_PATH"))

from pathlib import Path
from typing import Optional

from pycode.scripts.converting_rules import ConvertingValues
from pycode.types.mc_explorer import MCExplorer

### WRITE YOUR CODE HERE

basefolder = Path("/home/leonardo/Documents/unige/data/12-04-2024/38936_DIV77")
raw_folder = basefolder.joinpath("raw")
save_folder = basefolder.joinpath("converted")
mkdir(save_folder)

def rule(name) -> Optional[ConvertingValues]:
    f_ = name.find("_")
    i = name[:f_]
    i = int(i)
    t = name[f_+1:-3]
    return ConvertingValues(
            matrice = "38936",
            cond = "100E",
            div = "77",
            i = i,
            t = t,
            )
    pass

for file in listdir(raw_folder):
    if file.endswith(".h5"):
        converting_values = rule(file)
        index_rec = 0
        index_raw = 0
        index_dig = None
        index_events = None
        if converting_values.t == "basal":
            pass
        elif converting_values.t == "StimEl":
            index_events = 0
        else:
            index_dig = 1
        file = raw_folder.joinpath(file)
        print(file)
        explorer = MCExplorer(file)
        phase = explorer.convert_phase(index_rec, index_raw, index_dig, index_events)
        phase.peak_detection(2e-3, 2e-3, 8)
        phase.save(save_folder.joinpath(f"{converting_values.matrice}_{converting_values.cond}_DIV{converting_values.div}_{converting_values.t}_000{converting_values.i}.h5"))
