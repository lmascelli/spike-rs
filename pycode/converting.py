from os import listdir, mkdir
from pathlib import Path
from typing import Callable, Optional

from scipy.io import savemat

from pycode.handlers.phaseh5 import PhaseH5

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


class ConvertingValues:
    def __init__(self, matrice: str, cond: str, div: str, i: str, t: str):
        self.matrice = matrice
        self.cond = cond
        self.div = div
        self.i = i
        self.t = t

    def __str__(self) -> str:
        return f"""{{
        matrice: {self.matrice},
        cond: {self.cond},
        div: {self.div},
        i: {self.i},
        t: {self.t},
}}"""


def rule1(name: str) -> Optional[ConvertingValues]:
    """
    Example: 01_basal
    {
        matrix: 00000,
        cond: XXX,
        div: 00,
        i: 01,
        t: basal,
    }
    """
    try:
        i = name[: name.find("_")]
        t = name[name.find("_") + 1 : -3]
        return ConvertingValues("00000", "XXX", "00", i, t)
    except Exception as _e:
        return None


def rule2(name: str) -> Optional[ConvertingValues]:
    """
    Example: 2024-04-11T14-31-1938940_100E_DIV77_nbasal_0001_E-00155.h5
    {
        matrix: 38940,
        cond: 100E,
        div: 77,
        i: 01,
        t: nbasal,
    }
    """
    try:
        first_ = name.find("_") + 1
        matrice = name[first_ - 6 : first_ - 1]
        second_ = name.find("_", first_) + 1
        cond = name[first_ : second_ - 1]
        third_ = name.find("_", second_) + 1
        div = name[name.find("DIV") + 3 : third_ - 1]
        fourth_ = name.find("_", third_) + 1
        t = name[third_ : fourth_ - 1]
        fifth_ = name.find("_", fourth_) + 1
        i = str(int(name[fourth_ : fifth_ - 1]))
        return ConvertingValues(matrice, cond, div, f"000{i}", t)
    except Exception as _e:
        return None


def _mkdir(path: Path) -> Optional[Path]:
    try:
        mkdir(path)
        return path
    except Exception as e:
        if path.exists():
            return path
        else:
            print(f"Error creating folder {path}: {e}")
            return None


def convert_recording_folder_to_mat(
    source: Path,
    dest: Optional[Path],
    converting_rule: Callable[[str], Optional[ConvertingValues]],
):
    if dest is None:
        dest = source

    num_files = 0
    cur_file = 0
    for file in listdir(source):
        if file.endswith(".h5"):
            num_files = num_files + 1

    for file in listdir(source):
        if file.endswith(".h5"):
            cur_file = cur_file + 1
            print(f"Converting ({cur_file}/{num_files}): {file}")
            converting_values = converting_rule(file)
            file = source.joinpath(file)
            phase = PhaseH5(f"{file.absolute()}")

            if converting_values is not None and phase is not None:
                matrice = converting_values.matrice
                cond = converting_values.cond
                div = converting_values.div
                t = converting_values.t
                i = converting_values.i

                base_folder = _mkdir(dest.joinpath(matrice))
                if base_folder is not None:
                    raw_files_root = _mkdir(base_folder.joinpath("Mat_files"))
                    raw_files_folder = _mkdir(
                        raw_files_root.joinpath(f"{matrice}_{cond}_DIV{div}_{t}_{i}")
                    )
                    _mkdir(raw_files_folder)

                    # save all raw electrodes data
                    for label in phase.labels():
                        raw_file_name = raw_files_folder.joinpath(
                            f"{matrice}_{cond}_DIV{div}_{t}_{i}_{label}.mat"
                        )
                        data = phase.raw_data(label)
                        if data is not None:
                            savemat(raw_file_name, {"data": data})

                    peaks_files_root = _mkdir(
                        base_folder.joinpath(f"{matrice}_PeakDetection")
                    )
                    peaks_files_folder = _mkdir(
                        peaks_files_root.joinpath(
                            f"ptrain_{matrice}_{cond}_DIV{div}_{t}_{i}"
                        )
                    )
                    _mkdir(peaks_files_folder)

                    for label in phase.labels():
                        peak_file_name = peaks_files_folder.joinpath(
                            f"ptrain_{matrice}_{cond}_DIV{div}_{t}_{i}_{label}.mat"
                        )
                        print(label)
                        peak_train = phase.peak_train(label)
                        if peak_train is not None:
                            savemat(peak_file_name, {"peak_train": peak_train})

                    digital_files_root = _mkdir(
                        base_folder.joinpath(f"{matrice}_Digital")
                    )
                    _mkdir(digital_files_root)

                    for index in range(phase.n_digitals()):
                        digital_file_name = digital_files_root.joinpath(
                            f"{matrice}_{t}_{i}.mat"
                        )
                        savemat(digital_file_name, {"digital": phase.digital(index)})

                    for index in range(phase.n_events()):
                        events = phase.events(index)
                        if events is not None:
                            stim_files_root = _mkdir(
                                base_folder.joinpath(f"{matrice}_IstStim")
                            )
                            stim_file_name = stim_files_root.joinpath(
                                f"{matrice}_{t}_{i}_{index}.mat"
                            )
                            _mkdir(stim_files_root)
                        savemat(stim_file_name, {"events": events})
            else:
                print(
                    f"convert_recording_folder_to_mat: failed to parse the converting values from file: {file}"
                )
