from typing import Optional
from pathlib import Path

from PySide6.QtWidgets import (QFileDialog)

###############################################################################
#
#                              GLOBAL VARIABLES
#
###############################################################################

ROOT = None

CURRENT_PATH = None
CURRENT_PHASE = None
CURRENT_PHASE_PATH = None

CURRENT_STATE = None

CURRENT_SELECTED_SIGNAL = None

CURRENT_MC_FILE = None

HISTO_BINS_NUMBER = 30

###############################################################################
#
#                              GLOBAL FUNCTIONS
#
###############################################################################

def open_mc_file(filename: Optional[Path]):
    global CURRENT_MC_FILE
    if filename is not None:
        CURRENT_MC_FILE = Path(filename).absolute()
    else:
        opened_file = QFileDialog.getOpenFileName(caption="Select the MultiChannel hdf5",
                                        filter="HDF5 (*.h5)")[0]
        if opened_file != "":
            CURRENT_MC_FILE = Path(opened_file).absolute()
