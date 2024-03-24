import pycode as pc
from pathlib import Path
import matplotlib.pyplot as plt
import numpy as np

# TODO

convert_filename = Path("/home/leonardo/Documents/unige/raw_data/14-03-2024/38891_DIV49/06-StimUS65.h5")

# TEST H5Content
content = pc.H5Content(convert_filename)
content.test()
print(content)
