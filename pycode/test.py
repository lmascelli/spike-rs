from mc_explorer import MCExplorer
from pathlib import Path
from os import listdir, mkdir, rename

base_path = Path("/home/leonardo/Documents/unige/raw data/12-04-2024/38886_DIV77/")
raw_path = base_path.joinpath('raw')
for raw in listdir(raw_path):
    raw_file = raw_path.joinpath(raw)
    explorer = MCExplorer(raw_file)
    raw_index = None
    digital_index = None
    event_index = None
    for (i, path) in explorer.list_analogs(0):
        print(explorer.analog_dims(0, i))
