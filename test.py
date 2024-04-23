import sys

sys.path.insert(0, '/home/leonardo/Documents/unige/spike-rs')

from pathlib import Path
from typing import Optional

from pycode.scripts.convert_phases_to_mat import convert_recording_folder_to_mat
from pycode.scripts.converting_rules import ConvertingValues

base_path = Path("/home/leonardo/Documents/unige/raw data/12-04-2024/38940_DIV77/hdf5/")

def rule(name: str) -> Optional[ConvertingValues]:
    print(name)
    return None

convert_recording_folder_to_mat(base_path, None, rule)
