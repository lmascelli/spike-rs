from pycode.converting import rule2, convert_recording_folder_to_mat
from pathlib import Path

PATH = "/home/leonardo/Documents/unige/data/18-07-2024/38927/raw"
PATH = Path(PATH)
convert_recording_folder_to_mat(PATH, PATH, rule2)
