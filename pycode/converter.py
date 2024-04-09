from mc_explorer import MCExplorer
import pycode as pc
from pathlib import Path
import matplotlib.pyplot as plt
import numpy as np

from PySide6.QtWidgets import QVBoxLayout, QWidget

class Explorer(QWidget):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        layout = QVBoxLayout()
        self.setLayout(layout)
