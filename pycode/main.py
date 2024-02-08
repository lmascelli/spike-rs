from sys import argv, exit
from os.path import getctime, getsize, realpath
from datetime import datetime
from pathlib import Path
from typing import Optional

from PySide6.QtCore import QDir, Qt, QUrl  # type: ignore
from PySide6.QtGui import QAction
from PySide6.QtWidgets import (QApplication, QCheckBox, QDialog, QFileDialog,
                               QFileSystemModel, QGroupBox, QLabel,
                               QMainWindow, QMenu, QMenuBar, QMessageBox,
                               QPushButton, QSplitter, QStackedWidget,
                               QTextBrowser, QTreeView, QVBoxLayout, QWidget)

import spyke_rs as sp


###############################################################################
#
#                              GLOBAL VARIABLES
#.globals
###############################################################################

# handlers
ROOT = None                                      
ERROR_MSGBOX = None

CURRENT_PATH = None
CURRENT_PHASE = None
CURRENT_PHASE_PATH = None

###############################################################################
#
#                              GLOBAL FUNCTIONS
#
###############################################################################

def open_recordings():
    CURRENT_PATH = Path(QFileDialog.getExistingDirectory(
                                            caption="Select the phase file"))
    str_path = str(CURRENT_PATH.absolute())
    ROOT.tree.model.setRootPath(str_path)
    ROOT.tree.setRootIndex(ROOT.tree.model.index(str_path))
    switch_state('INSPECT_RECORDINGS_FOLDER')

def open_phase():
    if CURRENT_PHASE_PATH is not None:
        CURRENT_PHASE = sp.load_phase(str(CURRENT_PHASE_PATH))
        if CURRENT_PHASE is None:
            ERROR_MSGBOX.setText(f"Failed to load {CURRENT_PHASE_PATH}")
            ERROR_MSGBOX.exec()
        else:
            ERROR_MSGBOX.setText(f"{CURRENT_PHASE.digitals_num}")
            ERROR_MSGBOX.exec()
    else:
        ERROR_MSGBOX.setText(f"No phase path selected")
        ERROR_MSGBOX.exec()


def peak_detection():
    pass


###############################################################################
#
#                                 GUI STATES
#.states
###############################################################################

def state_started():
    ROOT.tree.setVisible(False)
    ROOT.controls.open_phase_button.setEnabled(False)
    ROOT.controls.compute_peak_trains_button.setEnabled(False)
    viewer_widget = ROOT.viewer.widgets['None']
    ROOT.viewer.setCurrentIndex(viewer_widget[0])

def state_inspect_recordings_folder():
    ROOT.tree.setVisible(True)
    ROOT.controls.open_phase_button.setEnabled(False)
    ROOT.controls.compute_peak_trains_button.setEnabled(False)
    viewer_widget = ROOT.viewer.widgets[ 'PhaseInfo' ]
    ROOT.viewer.setCurrentIndex(viewer_widget[0])

def state_inspect_recordings_folder_phase_selected():
    # ROOT.tree.setVisible(True)                # not managed here
    ROOT.controls.open_phase_button.setEnabled(True)
    ROOT.controls.compute_peak_trains_button.setEnabled(False)

GUI_STATES = {
        'STARTED': state_started,
        'INSPECT_RECORDINGS_FOLDER': state_inspect_recordings_folder,
        'INSPECT_RECORDINGS_FOLDER_PHASE_SELECTED': state_inspect_recordings_folder_phase_selected,
        'INSPECT_PHASE': None,
        }

OLD_STATE = None
CURRENT_STATE = None

def switch_state(new_state: str):
    global OLD_STATE
    global CURRENT_STATE
    if new_state in GUI_STATES:
        OLD_STATE = CURRENT_STATE
        CURRENT_STATE = new_state
        GUI_STATES[CURRENT_STATE]()


###############################################################################
#
#                                 CONTROLS
#.controls
###############################################################################

class Controls(QWidget):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        layout = QVBoxLayout()
        self.setLayout(layout)

        # FILE OPERATIONS GROUP
        file_group = QGroupBox(title="File")
        file_layout = QVBoxLayout()
        file_layout.setAlignment(Qt.AlignmentFlag.AlignTop)
        file_group.setLayout(file_layout)

        open_recordings_button = QPushButton("Open recordings folder")
        open_recordings_button.clicked.connect(open_recordings)
        file_layout.addWidget(open_recordings_button)

        self.open_phase_button = QPushButton("Open phase")
        self.open_phase_button.clicked.connect(open_phase)
        file_layout.addWidget(self.open_phase_button)

        layout.addWidget(file_group)
        ####################

        # DATA OPERATIONS GROUP
        data_group = QGroupBox(title="Analysis")
        data_layout = QVBoxLayout()
        data_layout.setAlignment(Qt.AlignmentFlag.AlignTop)
        data_group.setLayout(data_layout)

        self.compute_peak_trains_button = QPushButton("Peak detection")
        self.compute_peak_trains_button.clicked.connect(peak_detection)
        data_layout.addWidget(self.compute_peak_trains_button)

        layout.addWidget(data_group)
        ####################



###############################################################################
#
#                                 FILE TREE
#
###############################################################################

class FileTree(QTreeView):
    class InfoH5:
        def __init__(self, name, size, date):
            self.name = name
            self.size = size
            self.date = date

    def __init__(self, root: Path, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)

        self.setVisible(False)
        self.model = QFileSystemModel()
        self.model.setNameFilters(["*.h5"])
        self.model.setNameFilterDisables(False)
        self.model.setRootPath(str(root.absolute()))
        self.setModel(self.model)
        self.setRootIndex(self.model.index(QDir.currentPath()))

        self.hideColumn(1)
        self.hideColumn(2)
        self.hideColumn(3)
        self.setColumnWidth(0, 200)

        def file_selection_changed(new_file, old_file):
            count = new_file.count()
            if count > 0:
                global CURRENT_PHASE_PATH
                model_index = new_file.indexes()[0]
                path = self.model.filePath(model_index)
                CURRENT_PHASE_PATH = Path(path)
                file = CURRENT_PHASE_PATH
                info_h5 = self.InfoH5(
                    file.name, f'{"%.2f" % (getsize(file) / 1024 / 1024)}'
                    ' MB', datetime.fromtimestamp(getctime(file))
                    .strftime('%Y-%m-%d %H:%M:%S'))
                ROOT.viewer.widgets['PhaseInfo'][1].set_h5_info(info_h5=info_h5)
                switch_state('INSPECT_RECORDINGS_FOLDER_PHASE_SELECTED')

        self.selectionChanged = file_selection_changed


###############################################################################
#
#                               CENTRAL VIEWER
#.viewer
###############################################################################

class PhaseInfo(QLabel):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)

    def set_h5_info(self, info_h5: Optional[FileTree] = None):
        if info_h5 is not None:
            content = f'''
File name:      {info_h5.name}
File size:      {info_h5.size}
Creation date:  {info_h5.date}
            '''
            self.setText(content)


class PhaseView(QWidget):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)


class Viewer(QStackedWidget):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)

        self.widgets = {
                'None'      : (0, QWidget(self)),
                'PhaseInfo' : (1, PhaseInfo(self)),
                'PhaseView' : (2, PhaseView(self)),
                }

        for _, (index, widget) in self.widgets.items():
            self.insertWidget(index, widget)



###############################################################################
#
#                                  MAIN WINDOW
#
###############################################################################

class Main(QMainWindow):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)


        ############ WIDGETS ###########
        # SPLITTER
        self.splitter = QSplitter()
        self.setCentralWidget(self.splitter)

        self.splitter.setWindowTitle("Hdf5 phases explorator")
        self.splitter.setHandleWidth(1)
        self.splitter.setStyleSheet("QSplitter::handle{background-color:rgb(0, 0, 0);}")

        # FILE TREE
        self.tree = FileTree(Path('.'))
        self.splitter.addWidget(self.tree)

        # VIEWER
        self.viewer = Viewer()
        self.splitter.addWidget(self.viewer)

        # CONTROLS
        self.controls = Controls()
        self.splitter.addWidget(self.controls)

    def resizeEvent(self, a):
        min_width = 800
        min_height = 600
        tree_min_size = 300
        tree_max_size = 400
        tree_parts = 3
        controls_min_size = 100
        controls_max_size = 300
        controls_parts = 3

        cur_geometry = self.splitter.geometry()
        w = cur_geometry.width()
        tree_size = w // 10 * tree_parts
        if tree_size > tree_max_size:
            tree_size = tree_max_size
        elif tree_size < tree_min_size:
            tree_size = tree_min_size
        controls_size = w // 10 * controls_parts
        if controls_size > controls_max_size:
            controls_size = controls_max_size
        elif controls_size < controls_min_size:
            controls_size = controls_min_size
        if cur_geometry.width() < (tree_size + controls_size):
            cur_geometry.setWidth(tree_size + controls_size)
            self.splitter.setGeometry(cur_geometry)

        viewer_size = cur_geometry.width() - tree_size - controls_size
        self.splitter.setSizes([tree_size, viewer_size, controls_size])

        if cur_geometry.width() < min_width:
            cur_geometry.setWidth(min_width)
            self.splitter.setGeometry(cur_geometry)
        if cur_geometry.height() < min_height:
            cur_geometry.setHeight(min_height)
            self.splitter.setGeometry(cur_geometry)


###############################################################################
#
#                                 ENTRY POINT
#
###############################################################################

if __name__ == "__main__":
    app = QApplication(argv)
    win = Main()
    ROOT = win
    ERROR_MSGBOX = QMessageBox(ROOT)
    switch_state('STARTED')
    win.showMaximized()
    exit(app.exec())
