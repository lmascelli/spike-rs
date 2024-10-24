from typing import Dict, List, Tuple, cast
import sys
from os import listdir

import PySide6.QtCore as qtc
import PySide6.QtWidgets as qtw
import PySide6.QtGui as qtg

from gui.main import Ui_MainWindow
from gui.project_view import Ui_ProjectView
from pycode.project import Batch, Project
from pathlib import Path

BATCHES_DATA_INDEX = 1

class ProjectView(qtw.QWidget):
    def __init__(self, project: Project, project_index: int):
        super().__init__()
        self.project = project
        self.pw = Ui_ProjectView()
        self.pw.setupUi(self)
        self.project_index = project_index
        self.pw.batches_view.currentItemChanged.connect(self.update_phases)

    def update_lists(self):
        self.pw.batches_view.clear()
        for i, batch in enumerate(self.project.batches):
            item = qtw.QListWidgetItem(listview=self.pw.batches_view)
            item.setText(batch.name)
            item.setData(qtc.Qt.ItemDataRole.UserRole, i)

    def update_phases(self, current: qtw.QWidgetItem, previous: qtw.QWidgetItem):
        batch_index = self.pw.batches_view.currentIndex().row()
        for i, phase in enumerate(self.project.batches[batch_index].phases):
            item = qtw.QListWidgetItem(listview=self.pw.phases_view)
            item.setText(phase.name)
            item.setData(qtc.Qt.ItemDataRole.UserRole, i)


class MainWindow(qtw.QMainWindow):
    def __init__(self):
        super().__init__()
        self.mw = Ui_MainWindow()
        self.mw.setupUi(self)
        self.tabs: Dict[int, Tuple[qtw.QWidget, int]] = {}

        self.project_counter = 0

        self.projects: List[Project] = []
        self.current_project = -1

        # CONNECT MENU ACTIONS
        self.mw.actionExit.triggered.connect(self.quit)
        # self.mw.action_Open_Project.triggered.connect(self.openFolder)
        self.mw.action_CloseTab.triggered.connect(self.closeTab)
        self.mw.action_New_Project.triggered.connect(self.newProject)
        self.mw.action_Add_batch.triggered.connect(self.addBatchFolder)
        self.mw.w_tab.tabCloseRequested.connect(self.closeTab)

    def newProject(self):
        self.current_project = self.current_project + 1
        self.projects.append(Project())
        w_project = ProjectView(
            self.projects[self.current_project], self.current_project
        )
        self.addTab(f"New Project {self.project_counter}", w_project)
        self.project_counter = self.project_counter + 1

    def addBatchFolder(self):
        folder = Path(
            qtw.QFileDialog.getExistingDirectory(caption="Select batch folder")
        )
        if folder.exists() and folder.is_dir():
            batch = Batch(folder.name)
            for file in listdir(folder):
                if file.endswith(".h5"):
                    batch.add_phase(folder.joinpath(file))
            self.projects[self.current_project].batches.append(batch)
            cast(ProjectView, self.tabs[self.current_project][0]).update_lists()
        else:
            pass

    def addTab(self, name: str, widget: qtw.QWidget):
        self.tabs[self.current_project] = (widget, self.mw.w_tab.currentIndex() + 1)
        self.mw.w_tab.addTab(widget, name)
        self.mw.w_tab.setCurrentIndex(self.tabs[self.current_project][1])

    def closeTab(self):
        ti = self.mw.w_tab.currentIndex()
        self.mw.w_tab.removeTab(ti)

    def quit(self):
        self.close()


def run():
    app = qtw.QApplication(sys.argv)
    win = MainWindow()
    win.show()
    exit(app.exec())
