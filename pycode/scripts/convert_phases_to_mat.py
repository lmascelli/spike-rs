from os import listdir, mkdir
from pathlib import Path
from typing import Optional

from scipy.io import savemat

from pycode.types.pyphase import PyPhase

"""
Utility script for converting a folder containing the phase files of a recording
to a filesystem structure compatible with existing SpyCode scripts.

Example:
---------
filesystem before conversion:
    [BATCH_NAME]:
        - phase1.h5
        - phase2.h5
        - ...

filesystem after conversion:
    [BATCH_NAME]:
        - [BATCH_NAME]_Mat_files:
            - [BATCH_NAME]_[COND]_DIV[DIV]_[TYPE]_000[N_PHASE]:
                - [BATCH_NAME]_[COND]_DIV[DIV]_[TYPE]_000[N_PHASE]_[EL1_LABEL].mat:
                    {
                        "data": raw_data
                    }
                - [BATCH_NAME]_[COND]_DIV[DIV]_[TYPE]_000[N_PHASE]_[EL2_LABEL].mat:
                    {
                        "data": raw_data
                    }
                - ...
        - [BATCH_NAME]_Digital:
            - [BATCH_NAME]_[TYPE]_000[N_PHASE].mat:
                {
                    "digital": raw_data of digital
                }
        - [BATCH_NAME]_PeakDetection:
            - ptrain_[BATCH_NAME]_[COND]_DIV[DIV]_[TYPE]_000[N_PHASE]:
                - ptrain_[BATCH_NAME]_[COND]_DIV[DIV]_[TYPE]_000[N_PHASE]_[EL1_LABEL].mat:
                    {
                        "peak_train": array of peak samples
                    }
                - ptrain_[BATCH_NAME]_[COND]_DIV[DIV]_[TYPE]_000[N_PHASE]_[EL2_LABEL].mat:
                    {
                        "peak_train": array of peak samples
                    }
                - ...
        - [BATCH_NAME]_IstStim:
            - [BATCH_NAME]_[TYPE]_000[N_PHASE].mat:
                {
                    "events": [vector_of_starts, vector_of_ends]
                }
"""


def convert_recording_folder_to_mat(source: Path, dest: Optional[Path]):
    if dest is None:
        dest = source

    for file in listdir(source):
        phase = PyPhase.from_file(source.joinpath(file))
        print(phase)