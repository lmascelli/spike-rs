import os
import subprocess
from pathlib import Path
from typing import Optional, Tuple

import pycode.pycode_rs as sp


def convert_mc_acquisition(
    source: Path,
    mcdataconv_path: Path,
    wine_prefix: Optional[str] = None
    ) -> bool:
    if wine_prefix is not None:
        source_ = str(source.absolute())
        command = f'WINEPREFIX="{wine_prefix}" wine "{str(mcdataconv_path)}" -t hdf5 "z:{source_}"'
    else:
        command = f'{mcdataconv_path}  -t hdf5 "{source}"'

    print(command)
    # subprocess.run(command, shell=True, capture_output=True, text=True)


def convert_mc_h5_phase(source: Path, dest: Path) -> bool:
    """
    Convert an HDF5 file obtained from MultiChannel Data Manager into a PyClass
    readable file

    Args:
        source (Path): path of the source file
        dest (Path): path of the destination file

    Returns:
        None
    """
    result = sp.convert_mc_h5_file(str(source.absolute()), str(dest.absolute()))
    if result == 0:
        return True
    else:
        return False
