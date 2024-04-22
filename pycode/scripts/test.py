from mc_explorer import MCExplorer
from pathlib import Path
from os import listdir, mkdir, rename
from converting_rules import rule2

base_path = Path("/home/leonardo/Documents/unige/raw data/12-04-2024/38940_DIV77/")
raw_path = base_path.joinpath('raw')

for raw in listdir(raw_path):
    raw_file = raw_path.joinpath(raw)
    explorer = MCExplorer(raw_file)
    explorer.convert_with_rule(rule2)

    x = input("continue? ")
    if not x.upper() == "Y":
        break
