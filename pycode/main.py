from sys import argv, exit
from os.path import getctime, getsize, isfile, normpath, realpath
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from scipy.io import savemat

import matplotlib.pyplot as plt
import numpy as np

from PySide6.QtCore import QDir, Qt, QUrl  # type: ignore
from PySide6.QtGui import QAction, QFont, QPainter, QPen, QPixmap
from PySide6.QtWidgets import (QApplication, QCheckBox, QDialog, QFileDialog,
    QFileSystemModel, QFormLayout, QGroupBox, QHBoxLayout, QHeaderView, QLabel,
    QLineEdit, QListWidget, QMainWindow, QMenu, QMenuBar, QMessageBox,
    QPushButton, QSizePolicy, QSplitter, QStackedWidget, QTextBrowser,
    QTreeView, QTreeWidget, QTreeWidgetItem, QVBoxLayout, QWidget)

import pycode as pc
import pycode_rs as sp


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

CURRENT_SELECTED_SIGNAL = None

HISTO_BINS_NUMBER = 50

###############################################################################
#
#                              GLOBAL FUNCTIONS
#
###############################################################################

# FILE STUFF
def open_recordings():
    CURRENT_PATH = Path(QFileDialog.getExistingDirectory(
        caption="Select the phase file"))
    str_path = str(CURRENT_PATH.absolute())
    ROOT.tree.model.setRootPath(str_path)
    ROOT.tree.setRootIndex(ROOT.tree.model.index(str_path))
    switch_state('INSPECT_RECORDINGS_FOLDER')

def open_phase():
    global CURRENT_PHASE
    if CURRENT_PHASE_PATH is not None:
        CURRENT_PHASE = pc.PyPhase(CURRENT_PHASE_PATH)
        if CURRENT_PHASE is None:
            ERROR_MSGBOX.setText(f"Failed to load {CURRENT_PHASE_PATH}")
            ERROR_MSGBOX.exec()
        else:
            switch_state('INSPECT_PHASE')

    else:
        ERROR_MSGBOX.setText(f"No phase path selected")
        ERROR_MSGBOX.exec()

def save_phase():
    if CURRENT_PHASE is not None:
        save_file = Path(QFileDialog.getSaveFileName(
            filter="hdf5 (*.h5)",
            caption="Select the phase file")[0]).absolute()
        CURRENT_PHASE.save(save_file)
    else:
        ERROR_MSGBOX.setText(f"No phase loaded")
        ERROR_MSGBOX.exec()

def convert_from_mc_h5():
    ConvertFromMultichannelH5Dialog().exec()

def convert_phase():
    if CURRENT_PHASE is not None:
        save_folder = Path(QFileDialog.getExistingDirectory(
            caption="Select the export folder")).absolute()
        for label in CURRENT_PHASE.channel_labels:
            savemat(f"{str(save_folder)}/{label}.mat", {
                'data': CURRENT_PHASE.get_raw_data(label),
            })
    else:
        ERROR_MSGBOX.setText(f"No phase loaded")
        ERROR_MSGBOX.exec()

# PLOT STUFF
def plot_rasterplot():
    if CURRENT_PHASE is not None:
        electrodes = []
        spikes = []
        for label in CURRENT_PHASE.channel_labels:
            electrodes.append(label)
            spikes.append(CURRENT_PHASE.get_peaks_train(label)[1][:])
        plt.eventplot(spikes)
        plt.show()
    else:
        ERROR_MSGBOX.setText(f"No phase path selected")
        ERROR_MSGBOX.exec()

def plot_signal():
    if CURRENT_PHASE is not None:
        if CURRENT_SELECTED_SIGNAL is not None:
            data = None
            t, label = CURRENT_SELECTED_SIGNAL
            if t == 'digital':
                data = CURRENT_PHASE.get_digital(int(label))
            elif t == 'raw_data':
                data = CURRENT_PHASE.get_raw_data(label)
            else:
                return
            plt.plot(data)
            if ROOT.controls.plot_with_peaks_cb.isChecked():
                peak_values, peak_times = CURRENT_PHASE.get_peaks_train(label)
                plt.scatter(peak_times, peak_values, color="red")
            plt.show()
    else:
        ERROR_MSGBOX.setText(f"No phase path selected")
        ERROR_MSGBOX.exec()

def plot_peaks_histogram():
    if CURRENT_PHASE is not None:
        if CURRENT_SELECTED_SIGNAL is not None:
            data = None
            t, label = CURRENT_SELECTED_SIGNAL
            if t == 'peak_train':
                data = CURRENT_PHASE.get_peaks_bins(HISTO_BINS_NUMBER)[label]
            else:
                return
            ticks_values = np.linspace(data[1], data[2], HISTO_BINS_NUMBER + 1
                                    ).tolist()
            ticks = []
            for tick in ticks_values:
                ticks.append(f"{tick:3.2e}")
            plt.bar(ticks, data[0])
            plt.xticks(rotation=45)
            plt.show()
    else:
        ERROR_MSGBOX.setText(f"No phase path selected")
        ERROR_MSGBOX.exec()

# ANALYSIS STUFF
class ClearOverThresholdDialog():
    pass

def clear_peaks_over_threshold():
    if CURRENT_PHASE is not None:
        ClearOverThresholdDialog().exec()

    else:
        ERROR_MSGBOX.setText(f"No phase loaded")
        ERROR_MSGBOX.exec()

def peak_detection(peak_duration: float, refractary_time: float, n_devs: float):
    if CURRENT_PHASE is not None:
        switch_state('PEAK_DETECTION_DONE')
        CURRENT_PHASE.peak_detection(peak_duration, refractary_time, n_devs)

    else:
        ERROR_MSGBOX.setText(f"No phase loaded")
        ERROR_MSGBOX.exec()

def create_interval():
    IntervalCreationDialog().exec()


###############################################################################
#
#                                 GUI STATES
#.states
###############################################################################

def state_started():
    ROOT.tree.setVisible(False)
    ROOT.controls.open_phase_button.setEnabled(False)
    ROOT.controls.compute_peak_trains_button.setEnabled(False)
    ROOT.controls.convert_phase_button.setEnabled(False)
    ROOT.controls.create_interval_button.setEnabled(False)
    ROOT.controls.plot_signal_button.setEnabled(False)
    ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    ROOT.controls.plot_rasterplot_button.setEnabled(False)
    ROOT.controls.plot_with_intervals_cb.setEnabled(False)
    ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    ROOT.controls.clear_peaks_over_threshold_button.setEnabled(False)
    viewer_widget = ROOT.viewer.widgets['None']
    ROOT.viewer.setCurrentIndex(viewer_widget[0])

def state_inspect_recordings_folder():
    ROOT.tree.setVisible(True)
    ROOT.controls.open_phase_button.setEnabled(False)
    ROOT.controls.compute_peak_trains_button.setEnabled(False)
    ROOT.controls.plot_rasterplot_button.setEnabled(False)
    ROOT.controls.plot_with_intervals_cb.setEnabled(False)
    ROOT.controls.convert_phase_button.setEnabled(False)
    ROOT.controls.create_interval_button.setEnabled(False)
    ROOT.controls.plot_signal_button.setEnabled(False)
    ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    ROOT.controls.clear_peaks_over_threshold_button.setEnabled(False)
    viewer_widget = ROOT.viewer.widgets[ 'PhaseInfo' ]
    ROOT.viewer.setCurrentIndex(viewer_widget[0])

def state_inspect_recordings_folder_phase_selected():
    # ROOT.tree.setVisible(True)                # not managed here
    ROOT.controls.open_phase_button.setEnabled(True)
    ROOT.controls.convert_phase_button.setEnabled(False)
    ROOT.controls.plot_rasterplot_button.setEnabled(False)
    ROOT.controls.plot_with_intervals_cb.setEnabled(False)
    ROOT.controls.compute_peak_trains_button.setEnabled(False)
    ROOT.controls.plot_signal_button.setEnabled(False)
    ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    ROOT.controls.create_interval_button.setEnabled(False)
    ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    ROOT.controls.clear_peaks_over_threshold_button.setEnabled(False)

def state_inspect_phase():
    # ROOT.tree.setVisible(True)                # not managed here
    ROOT.controls.open_phase_button.setEnabled(True)
    ROOT.controls.compute_peak_trains_button.setEnabled(True)
    ROOT.controls.convert_phase_button.setEnabled(True)
    ROOT.controls.create_interval_button.setEnabled(False)
    ROOT.controls.plot_signal_button.setEnabled(False)
    ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    ROOT.controls.plot_rasterplot_button.setEnabled(True)
    ROOT.controls.plot_with_intervals_cb.setEnabled(True)
    ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    ROOT.controls.clear_peaks_over_threshold_button.setEnabled(True)
    phase_info = ROOT.viewer.widgets['PhaseView']
    ROOT.viewer.setCurrentIndex(phase_info[0])
    phase_info[1].update_data()

def state_update_peaks():
    ROOT.controls.open_phase_button.setEnabled(True)
    ROOT.controls.compute_peak_trains_button.setEnabled(False)
    ROOT.controls.convert_phase_button.setEnabled(True)
    ROOT.controls.plot_signal_button.setEnabled(False)
    ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    ROOT.controls.clear_peaks_over_threshold_button.setEnabled(True)
    ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    phase_info = ROOT.viewer.widgets['PhaseView']
    ROOT.viewer.setCurrentIndex(phase_info[0])
    phase_info[1].update_peaks()

def state_selected_signal():
    ROOT.controls.plot_signal_button.setEnabled(True)
    ROOT.controls.plot_with_peaks_cb.setEnabled(True)
    ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    ROOT.controls.create_interval_button.setEnabled(False)
    if CURRENT_SELECTED_SIGNAL is not None and CURRENT_SELECTED_SIGNAL[0] == 'digital':
        switch_state('SELECTED_DIGITAL')

def state_selected_peak_train():
    ROOT.controls.plot_signal_button.setEnabled(False)
    ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    ROOT.controls.plot_peaks_histogram_button.setEnabled(True)
    ROOT.controls.create_interval_button.setEnabled(False)

def state_selected_digital():
    ROOT.controls.plot_signal_button.setEnabled(True)
    ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    ROOT.controls.create_interval_button.setEnabled(True)

def state_peak_detection_done():
    phase_info = ROOT.viewer.widgets['PhaseView']
    ROOT.viewer.setCurrentIndex(phase_info[0])
    phase_info[1].update_peaks()

GUI_STATES = {
    'STARTED': state_started,
    'INSPECT_RECORDINGS_FOLDER': state_inspect_recordings_folder,
    'INSPECT_RECORDINGS_FOLDER_PHASE_SELECTED': state_inspect_recordings_folder_phase_selected,
    'INSPECT_PHASE': state_inspect_phase,
    'SELECTED_SIGNAL': state_selected_signal,
    'SELECTED_DIGITAL': state_selected_digital,
    'SELECTED_PEAK_TRAIN': state_selected_peak_train,
    'UPDATE_PEAKS': state_update_peaks,
    'PEAK_DETECTION_DONE': state_peak_detection_done,
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

        self.save_phase_button = QPushButton("Save phase")
        self.save_phase_button.clicked.connect(save_phase)
        file_layout.addWidget(self.save_phase_button)

        self.convert_from_mc_h5_button = QPushButton("Convert MultiChannel hdf5 to phase")
        self.convert_from_mc_h5_button.clicked.connect(convert_from_mc_h5)
        file_layout.addWidget(self.convert_from_mc_h5_button)

        self.convert_phase_button = QPushButton("Convert phase to .mat files")
        self.convert_phase_button.clicked.connect(convert_phase)
        file_layout.addWidget(self.convert_phase_button)

        layout.addWidget(file_group)
        ####################

        # INTERVALS GROUP
        intervals_group = QGroupBox(title="Intervals")
        intervals_layout = QVBoxLayout()
        intervals_layout.setAlignment(Qt.AlignmentFlag.AlignTop)
        intervals_group.setLayout(intervals_layout)

        self.create_interval_button = QPushButton("Create interval")
        self.create_interval_button.clicked.connect(create_interval)
        intervals_layout.addWidget(self.create_interval_button)

        layout.addWidget(intervals_group)

        ####################

        # PLOT GROUP
        plot_group = QGroupBox(title="Plot")
        plot_layout = QVBoxLayout()
        plot_layout.setAlignment(Qt.AlignmentFlag.AlignTop)
        plot_group.setLayout(plot_layout)

        self.plot_signal_button = QPushButton("Plot Signal")
        self.plot_with_peaks_cb = QCheckBox(text="Spikes in signal plot")
        self.plot_signal_button.clicked.connect(plot_signal)
        plot_layout.addWidget(self.plot_signal_button)
        plot_layout.addWidget(self.plot_with_peaks_cb)

        self.plot_peaks_histogram_button = QPushButton("Peaks histogram")
        self.plot_peaks_histogram_button.clicked.connect(plot_peaks_histogram)
        plot_layout.addWidget(self.plot_peaks_histogram_button)

        self.plot_rasterplot_button = QPushButton("Plot rasterplot")
        self.plot_with_intervals_cb = QCheckBox(text="Highlight intervals of selected digital")
        self.plot_rasterplot_button.clicked.connect(plot_rasterplot)
        plot_layout.addWidget(self.plot_rasterplot_button)
        plot_layout.addWidget(self.plot_with_intervals_cb)
        layout.addWidget(plot_group)
        ####################

        # DATA OPERATIONS GROUP
        data_group = QGroupBox(title="Analysis")
        data_layout = QVBoxLayout()
        data_layout.setAlignment(Qt.AlignmentFlag.AlignTop)
        data_group.setLayout(data_layout)

        self.compute_peak_trains_button = QPushButton("Peak detection")
        self.compute_peak_trains_button.clicked.connect(lambda: PeakDetectionDialog().exec())
        data_layout.addWidget(self.compute_peak_trains_button)

        self.clear_peaks_over_threshold_button = QPushButton("Clear peaks over threshold")
        self.clear_peaks_over_threshold_button.clicked.connect(clear_peaks_over_threshold)
        data_layout.addWidget(self.clear_peaks_over_threshold_button)

        layout.addWidget(data_group)
        ####################


###############################################################################
#
#                                 DIALOGS
#.dialogs
###############################################################################


class IntervalCreationDialog(QDialog):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        self.setGeometry(200, 200, 800, 600)
        layout = QVBoxLayout()

        # graphical representation
        self.draw_widget = QLabel()
        self.draw_widget.setPixmap(QPixmap(750, 350))
        layout.addWidget(self.draw_widget)

        # controls form
        controls_widget = QWidget()
        controls_layout = QFormLayout()
        self.offset_start_edit = QLineEdit(text="0")
        self.offset_end_edit = QLineEdit(text="0")
        self.interval_perc_edit = QLineEdit(text="0")
        controls_layout.addRow("offset start", self.offset_start_edit)
        controls_layout.addRow("offset end", self.offset_end_edit)
        controls_layout.addRow("interval percentage", self.interval_perc_edit)
        controls_widget.setLayout(controls_layout)
        layout.addWidget(controls_widget)

        # controls buttons

        buttons_widget = QWidget()
        buttons_layout = QHBoxLayout()
        buttons_widget.setLayout(buttons_layout)

        draw_button = QPushButton("Draw")
        buttons_layout.addWidget(draw_button)
        draw_button.clicked.connect(self.draw_actual)
        create_button = QPushButton("Create")
        buttons_layout.addWidget(create_button)
        cancel_button = QPushButton("Cancel")
        cancel_button.clicked.connect(self.close)
        buttons_layout.addWidget(cancel_button)

        layout.addWidget(buttons_widget)

        self.setLayout(layout)
        self.draw_base()

    def draw_base(self):
        self.canvas = self.draw_widget.pixmap()
        self.canvas.fill(Qt.white)
        painter = QPainter(self.canvas)
        pen = QPen()
        pen.setColor(Qt.blue)
        pen.setWidth(5)
        painter.setPen(pen)
        painter.drawLine(100, 250, 250, 250)
        painter.drawLine(250, 250, 250, 100)
        painter.drawLine(250, 100, 500, 100)
        painter.drawLine(500, 100, 500, 250)
        painter.drawLine(500, 250, 650, 250)
        font = QFont()
        font.setPointSize(20)
        painter.setFont(font)
        pen.setColor(Qt.green)
        painter.setPen(pen)
        painter.drawText(250, 280, "Start")
        painter.drawText(500, 280, "End")
        painter.drawText(350, 90, "Interval")
        self.draw_widget.setPixmap(self.canvas)
        
    def draw_actual(self):
        self.draw_base()
        self.offset_start = -10
        self.offset_end = -10
        self.interval_perc = 100
        try:
            self.offset_start = float(self.offset_start_edit.text())
            self.offset_end = float(self.offset_end_edit.text())
            self.interval_perc = float(self.interval_perc_edit.text())
        except Exception as e:
            ERROR_MSGBOX.setText(f"{e}")
            ERROR_MSGBOX.exec()


        self.canvas = self.draw_widget.pixmap()
        painter = QPainter(self.canvas)
        pen = QPen()
        pen.setColor(Qt.red)
        pen.setWidth(5)
        painter.setPen(pen)
        painter.drawLine(100+int(self.offset_start), 250, 250+int(self.offset_start), 250)
        painter.drawLine(250+int(self.offset_start), 250, 250+int(self.offset_start), 100)
        painter.drawLine(250+int(self.offset_start), 100, 500+int(self.offset_start), 100)
        painter.drawLine(500+int(self.offset_start), 100, 500+int(self.offset_start), 250)
        painter.drawLine(500+int(self.offset_start), 250, 650+int(self.offset_start), 250)
        self.draw_widget.setPixmap(self.canvas)
        

class ConvertFromMultichannelH5Dialog(QDialog):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        self.setGeometry(200, 200, 700, 300)
        layout = QFormLayout()

        self.source_label = QLineEdit()
        layout.addRow("Source file:", self.source_label)
        open_source_button = QPushButton(text="Choose file")
        open_source_button.clicked.connect(lambda: self.source_label.setText(
            QFileDialog.getOpenFileName(caption="Select the MultiChannel hdf5",
                                        filter="HDF5 (*.h5)")[0]
        ))
        layout.addRow("Search in the filesystem", open_source_button)
        self.dest_label = QLineEdit()
        layout.addRow("Destination file:", self.dest_label)
        open_dest_button = QPushButton(text="Choose file")
        open_dest_button.clicked.connect(lambda: self.dest_label.setText(
            QFileDialog.getSaveFileName(caption="Enter the destination hdf5",
                                        filter="HDF5 (*.h5)")[0]
        ))
        layout.addRow("Search in the filesystem", open_dest_button)
        convert_button = QPushButton("Convert")
        convert_button.clicked.connect(self.convert)
        layout.addRow("", convert_button)
        cancel_button = QPushButton("Cancel")
        cancel_button.clicked.connect(self.close)
        layout.addRow("", cancel_button)

        self.setLayout(layout)

    def convert(self):
        try:
            source = Path(self.source_label.text())
            dest = Path(self.dest_label.text())
            if pc.convert_mc_h5_phase(source, dest):
                return
            else:
                ERROR_MSGBOX.exec()
                ERROR_MSGBOX.setText(f"Failed converting the selected file")

        except Exception as e:
            ERROR_MSGBOX.setText(f"Failed parsing the source or destination files {e}")
            ERROR_MSGBOX.exec()


class PeakDetectionDialog(QDialog):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        self.setGeometry(200, 200, 500, 300)
        layout = QVBoxLayout()

        form_widget = QWidget()
        form_layout = QFormLayout()
        form_widget.setLayout(form_layout)
        self.peak_duration_label = QLineEdit(text="2e-3")
        form_layout.addRow("Peak duration:", self.peak_duration_label)
        self.refractary_time_label = QLineEdit("2e-3")
        form_layout.addRow("Refractary time:", self.refractary_time_label)
        self.n_devs_label = QLineEdit("8")
        form_layout.addRow("Multiplier for the stdev to use as threshold:", self.n_devs_label)
        layout.addWidget(form_widget)
        
        compute_button = QPushButton("Compute")
        compute_button.clicked.connect(self.compute)
        layout.addWidget(compute_button)
        cancel_button = QPushButton("Cancel")
        cancel_button.clicked.connect(self.close)
        layout.addWidget(cancel_button)

        self.setLayout(layout)

    def compute(self):
        try:
            peak_duration = float(self.peak_duration_label.text())
            refractary_time = float(self.refractary_time_label.text())
            n_devs = float(self.n_devs_label.text())
            peak_detection(peak_duration, refractary_time, n_devs)
            self.close()

        except Exception as e:
            ERROR_MSGBOX.setText(f"Failed parsing the inputs {e}")
            ERROR_MSGBOX.exec()

class ClearOverThresholdDialog(QDialog):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        self.setGeometry(200, 200, 300, 300)
        layout = QHBoxLayout()

        controls = QWidget()
        controls_layout = QVBoxLayout()
        controls.setLayout(controls_layout)

        controls_layout.addWidget(QLabel("Clear threshold"))
        self.threshold_edit = QLineEdit()
        confirm_button = QPushButton(text="Clear")
        confirm_button.clicked.connect(self.confirm)
        cancel_button = QPushButton(text="Cancel")
        cancel_button.clicked.connect(self.close)
        layout.addWidget(controls)
        controls_layout.addWidget(self.threshold_edit)
        controls_layout.addWidget(confirm_button)
        controls_layout.addWidget(cancel_button)

        self.setLayout(layout)

    def confirm(self):
        threshold_value = float(self.threshold_edit.text());
        CURRENT_PHASE.clear_peaks_over_threshold(threshold_value);
        switch_state('UPDATE_PEAKS')
        self.close()


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

class DigialView(QTreeWidget):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        self.setHeaderLabels(['Index', 'Number of Samples', 
                              'Sampling Frequency'])

    def selectionChanged(self, new_item, old_item):
        if new_item.count() > 0:
            global CURRENT_SELECTED_SIGNAL
            index = new_item.indexes()[0].data()
            n_samples = new_item.indexes()[1].data()
            sampling_frequency = new_item.indexes()[2].data()

            CURRENT_SELECTED_SIGNAL = ('digital', index)
            print(CURRENT_PHASE.get_digital_intervals(int(index)))

            switch_state('SELECTED_SIGNAL')


class RawDatasView(QTreeWidget):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        self.setHeaderLabels(['Label', 'Number of Samples', 
                              'Sampling Frequency'])

    def selectionChanged(self, new_item, old_item):
        if new_item.count() > 0:
            global CURRENT_SELECTED_SIGNAL
            label = new_item.indexes()[0].data()
            n_samples = new_item.indexes()[1].data()
            sampling_frequency = new_item.indexes()[2].data()

            CURRENT_SELECTED_SIGNAL = ('raw_data', label)

            switch_state('SELECTED_SIGNAL')

class PeakTrainsView(QTreeWidget):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        self.setHeaderLabels(['Label', 'Number of Samples'])

    def selectionChanged(self, new_item, old_item):
        if new_item.count() > 0:
            global CURRENT_SELECTED_SIGNAL
            label = new_item.indexes()[0].data()
            n_samples = new_item.indexes()[1].data()

            CURRENT_SELECTED_SIGNAL = ('peak_train', label)

            switch_state('SELECTED_PEAK_TRAIN')


class PhaseView(QWidget):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        layout = QVBoxLayout()
        self.setLayout(layout)

        layout.addWidget(QLabel("Digitals"))
        self.digitals = DigialView(self)
        layout.addWidget(self.digitals)

        layout.addWidget(QLabel("Raw datas"))
        self.raw_datas = RawDatasView(self)
        layout.addWidget(self.raw_datas)

        layout.addWidget(QLabel("Peak trains"))
        self.peak_trains = PeakTrainsView(self)
        layout.addWidget(self.peak_trains)

    def update_peaks(self):
        self.peak_trains.clear()

        # peak trains
        ordered_dict = dict(sorted(CURRENT_PHASE.peak_train_lengths.items()))
        for (label, count) in ordered_dict.items():
            item = QTreeWidgetItem(self.peak_trains)
            item.setText(0, f"{label}")
            item.setText(1, f"{count}")

        for i in range(0, self.peak_trains.columnCount()):
            self.peak_trains.header().setSectionResizeMode(i, QHeaderView.ResizeToContents)

    def update_data(self):
        # clear previous tables
        self.digitals.clear()
        self.raw_datas.clear()
        self.peak_trains.clear()

        # digitals
        for i, d in enumerate(CURRENT_PHASE.digitals_lengths):
            item = QTreeWidgetItem(self.digitals)
            item.setText(0, f"{i}")
            item.setText(1, f"{d}")
            item.setText(2, f"{CURRENT_PHASE.sampling_frequency}")

        # raw datas
        ordered_dict = dict(sorted(CURRENT_PHASE.raw_data_lengths.items()))
        for (label, count) in ordered_dict.items():
            item = QTreeWidgetItem(self.raw_datas)
            item.setText(0, f"{label}")
            item.setText(1, f"{count}")
            item.setText(2, f"{CURRENT_PHASE.sampling_frequency}")

        # peak trains
        ordered_dict = dict(sorted(CURRENT_PHASE.peak_train_lengths.items()))
        for (label, count) in ordered_dict.items():
            item = QTreeWidgetItem(self.peak_trains)
            item.setText(0, f"{label}")
            item.setText(1, f"{count}")

        # resize tables columns
        for i in range(0, self.digitals.columnCount()):
            self.digitals.header().setSectionResizeMode(i, QHeaderView.ResizeToContents)
        for i in range(0, self.raw_datas.columnCount()):
            self.raw_datas.header().setSectionResizeMode(i, QHeaderView.ResizeToContents)
        for i in range(0, self.peak_trains.columnCount()):
            self.peak_trains.header().setSectionResizeMode(i, QHeaderView.ResizeToContents)


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
    win.setWindowTitle("PyCode")
    ROOT = win
    ERROR_MSGBOX = QMessageBox(ROOT)
    switch_state('STARTED')
    win.showMaximized()
    exit(app.exec())
