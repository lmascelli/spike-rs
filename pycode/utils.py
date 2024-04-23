import os
import subprocess
from pathlib import Path
from typing import Optional, Tuple

import pycode.pycode_rs as sp


def convert_mc_acquisition(
    source: Path, dest: Path, mcdataconv_path: Path, wine_prefix: Optional[str] = None
) -> bool:
    os.chdir(dest.parent)
    if wine_prefix is not None:
        source_ = str(source.absolute())
        command = f'WINEPREFIX={wine_prefix} wine {str(mcdataconv_path)} -t hdf5 "z:{source_}"'
    else:
        command = f'{mcdataconv_path}  -t hdf5 "{source}"'

    subprocess.run(command, shell=True, capture_output=True, text=True)


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


def check_valid_bin_size(interval: Tuple[int, int], bin_size: int) -> int:
    """
    Checks if a bin_size can divide the inteval without too much residue and,
    if not, provide a near bin_size that does it

    Args:
        interval (Tuple[int, int])
        bin_size (int)

    Returns:
        a valid bin size (int)
    """
    return sp.check_valid_bin_size(interval, bin_size)
