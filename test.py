from os import getenv, listdir
import sys

sys.path.insert(0, getenv("PYCODE_PATH"))

from pathlib import Path
from typing import Optional

from pycode.scripts.convert_phases_to_mat import convert_recording_folder_to_mat
from pycode.scripts.converting_rules import ConvertingValues

base_path = Path("/home/leonardo/Documents/unige/raw data/12-04-2024/38940_DIV77/hdf5/")

def rule(name: str) -> Optional[ConvertingValues]:
    """
    Example:
    38940_100E_DIV77_nbasal_0005.h5

    {
        matrice: 38949,
        cond:    100E,
        div:     77,
        type:    nbasal,
        i:       5,
    }

    """
    first_ = name.find('_')
    second_ = name.find('_', first_ + 1)
    third_ = name.find('_', second_ + 1)
    fourth_ = name.find('_', third_ + 1)
    matrice = name[0:first_]
    cond = name[first_+1:second_]
    div = name[second_+4:third_]
    t = name[third_+1:fourth_]
    i = f"000{int(name[fourth_+1:-3])}"

    return ConvertingValues(matrice, cond, div, i, t)


# convert_recording_folder_to_mat(base_path, None, rule)

for file in listdir(base_path):
    if file.endswith(".h5"):
        print(file)
        print(rule(file))
        print('--------------------------------------------------------------------------------')

"""
import pycode as pc
from mc_explorer import MCExplorer

from os import listdir
from pathlib import Path

BASE_DIR = Path("/run/media/leonardo/Crucial X6/unige/raw data/12-04-2024/Stimolazione/39488")
DEST_DIR = Path("/home/leonardo/Documents/unige/raw data/12-04-2024")
MC_CONV_PATH = Path("/home/leonardo/.local/share/wineprefixes/Unige/drive_c/Program\\ Files/MCDataConv/MCDataConv.exe")


for file in listdir(BASE_DIR):
    if file.endswith(".msrs"):
        file = Path(BASE_DIR).joinpath(file)
        print(file)

        pc.convert_mc_acquisition(file,
                                  DEST_DIR,
                                  MC_CONV_PATH,
                                  "/home/leonardo/.local/share/wineprefixes/Unige")
"""
