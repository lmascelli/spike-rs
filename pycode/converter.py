from PySide6.QtWidgets import (QGroupBox, QHBoxLayout, QLabel, QListWidget,
                               QPushButton, QVBoxLayout, QWidget)

from mc_explorer import MCExplorer
import globals

import matplotlib.pyplot as plt


class Explorer(QWidget):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        layout = QVBoxLayout()
        self.setLayout(layout)

        # recordings group
        recordings_group = QGroupBox(title="Recordings")
        recordings_layout = QHBoxLayout()
        recordings_group.setLayout(recordings_layout)
        self.recordings = QListWidget()

        self.recording_info = QLabel()

        recordings_btn_group = QGroupBox()
        recordings_btn_layout = QVBoxLayout()
        recordings_btn_group.setLayout(recordings_btn_layout)
        open_file_btn = QPushButton(text="Open file")
        open_file_btn.clicked.connect(self.open_mc_file)
        open_recording_btn = QPushButton(text="Open recording")
        recordings_btn_layout.addWidget(open_file_btn)
        recordings_btn_layout.addWidget(open_recording_btn)

        recordings_layout.addWidget(self.recordings)
        recordings_layout.addWidget(self.recording_info)
        recordings_layout.addWidget(recordings_btn_group)
        ##########

        # analogs group
        analogs_group = QGroupBox(title="Analogs")
        analogs_layout = QHBoxLayout()
        analogs_group.setLayout(analogs_layout)

        self.analogs = QListWidget()

        self.analog_info = QLabel()

        analogs_btn_group = QGroupBox()
        analogs_btn_layout = QVBoxLayout()
        analogs_btn_group.setLayout(analogs_btn_layout)
        pick_raw_btn = QPushButton(text="Pick as raw data")
        pick_digital_btn = QPushButton(text="Pick as raw digital")
        analogs_btn_layout.addWidget(pick_raw_btn)
        analogs_btn_layout.addWidget(pick_digital_btn)

        analogs_layout.addWidget(self.analogs)
        analogs_layout.addWidget(self.analog_info)
        analogs_layout.addWidget(analogs_btn_group)
        ##########

        # events group
        events_group = QGroupBox(title="Events")
        events_layout = QHBoxLayout()
        events_group.setLayout(events_layout)

        self.events = QListWidget()

        self.event_info = QLabel()

        events_btn_group = QGroupBox()
        events_btn_layout = QVBoxLayout()
        events_btn_group.setLayout(events_btn_layout)
        pick_el_stim_btn = QPushButton(text="Pick as el stim events")
        events_btn_layout.addWidget(pick_el_stim_btn)

        events_layout.addWidget(self.events)
        events_layout.addWidget(self.event_info)
        events_layout.addWidget(events_btn_group)
        ##########
        
        # convert group
        convert_group = QGroupBox(title="Convert")
        convert_layout = QHBoxLayout()
        convert_group.setLayout(convert_layout)

        self.convert_summary = QLabel()
        
        convert_btn = QPushButton(text="Convert")

        convert_layout.addWidget(self.convert_summary)
        convert_layout.addWidget(convert_btn)
        ##########

        layout.addWidget(recordings_group)
        layout.addWidget(analogs_group)
        layout.addWidget(events_group)
        layout.addWidget(convert_group)

    def open_mc_file(self):
        globals.open_mc_file(None)
        self.explorer = MCExplorer(globals.CURRENT_MC_FILE)
