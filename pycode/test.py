from mc_explorer import MCExplorer
import pycode as pc
from pathlib import Path
import matplotlib.pyplot as plt
import numpy as np

# TODO

convert_filename = Path("/home/leonardo/Documents/unige/raw data/raw_test.h5")

# TEST H5Content
content = MCExplorer(convert_filename)
