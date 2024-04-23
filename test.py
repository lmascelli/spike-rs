import sys

sys.path.insert(0, '/home/leonardo/Documents/unige/spike-rs')

from pathlib import Path
from typing import Optional

from pycode.scripts.convert_phases_to_mat import convert_recording_folder_to_mat
from pycode.scripts.converting_rules import ConvertingValues

base_path = Path("/home/leonardo/Documents/unige/raw data/12-04-2024/giorg/")
# test_path = Path("/home/leonardo/Documents/unige/raw data/12-04-2024/38940_DIV77/test/")

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

convert_recording_folder_to_mat(base_path, None, rule)
