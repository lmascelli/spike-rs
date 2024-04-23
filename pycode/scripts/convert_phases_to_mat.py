from os import listdir, mkdir
from pathlib import Path
from typing import Callable, Optional

from scipy.io import savemat

from ..types.pyphase import PyPhase
from ..scripts.converting_rules import ConvertingValues

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

    for file in listdir(source):
        if file.endswith(".h5"):
            converting_values = converting_rule(file)
            file = source.joinpath(file)
            phase = PyPhase.from_file(file.absolute())

            if converting_values is not None and phase is not None:

                matrice = converting_values.matrice
                cond = converting_values.cond
                div = converting_values.div
                t = converting_values.t
                i = converting_values.i

                base_folder = _mkdir(dest.joinpath(matrice))
                if base_folder is not None:
                    raw_files_root = _mkdir(base_folder.joinpath("Mat_files"))
                    raw_files_folder = _mkdir(raw_files_root.joinpath(f"{matrice}_{cond}_DIV{div}_{t}_{i}"))
                    _mkdir(raw_files_folder)

                    # save all raw electrodes data
                    for label in phase.channel_labels:
                        raw_file_name = raw_files_folder.joinpath(f"{matrice}_{cond}_DIV{div}_{t}_{i}_{label}.mat")
                        data = phase.get_raw_data(label)
                        if data is not None:
                            savemat(raw_file_name, {
                                "data": data
                                })

                    peaks_files_root = _mkdir(base_folder.joinpath(f"{matrice}_PeakDetection"))
                    peaks_files_folder = _mkdir(peaks_files_root.joinpath(f"ptrain_{matrice}_{cond}_DIV{div}_{t}_{i}"))
                    _mkdir(peaks_files_folder)

                    for label in phase.channel_labels:
                        peak_file_name = peaks_files_folder.joinpath(f"ptrain_{matrice}_{cond}_DIV{div}_{t}_{i}_{label}.mat")
                        peak_train = phase.get_peaks_train(label)
                        if peak_train is not None:
                            savemat(peak_file_name, {
                                "peak_train": peak_train
                                })

                    digital_files_root = _mkdir(base_folder.joinpath(f"{matrice}_Digital"))
                    _mkdir(digital_files_root)

                    for i, _ in enumerate(phase.digitals_lengths):
                        digital_file_name = digital_files_root.joinpath(f"{matrice}_{t}_{i}.mat")
                        savemat(digital_file_name, {
                            "digital": phase.get_digital(i)
                            })

                    events = phase.get_el_stim_intervals()
                    if events is not None:
                        stim_files_root = _mkdir(base_folder.joinpath(f"{matrice}_IstStim"))
                        stim_file_name = stim_files_root.joinpath(f"{matrice}_{t}_{i}.mat")
                        _mkdir(stim_files_root)
                        savemat(stim_file_name, {
                            "events": events
                            })



                        


            else:
                print(
                    f"convert_recording_folder_to_mat: failed to parse the converting values from file: {file}"
                )
