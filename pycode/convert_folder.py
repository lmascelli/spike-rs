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
