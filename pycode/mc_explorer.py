import pycode_rs as sp
from pathlib import Path

################################################################################
#                             MCExplorer
################################################################################

class MCExplorer:
    def __init__(self, filename: Path):
        self.explorer = sp.MCExplorer(str(filename.absolute()))
