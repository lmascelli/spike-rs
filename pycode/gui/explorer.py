from PySide6.QtWidgets import (QGroupBox, QHBoxLayout, QLabel, QListWidget,
                               QListWidgetItem, QPushButton, QVBoxLayout,
                               QWidget)

from ..types.mc_explorer import MCExplorer
from . import globals


class Explorer(QWidget):
    def __init__(self, *kargs, **kwargs):
        super().__init__(*kargs, **kwargs)
        layout = QVBoxLayout()
        self.setLayout(layout)

        self.recording_index = None
        self.raw_stream_index = None
        self.digital_stream_index = None

        # recordings group
        recordings_group = QGroupBox(title="Recordings")
        recordings_layout = QHBoxLayout()
        recordings_group.setLayout(recordings_layout)
        self.recordings = QListWidget()
        self.recordings.selectionChanged = self.selected_recording

        self.recording_info = QLabel()

        recordings_btn_group = QGroupBox()
        recordings_btn_layout = QVBoxLayout()
        recordings_btn_group.setLayout(recordings_btn_layout)
        open_file_btn = QPushButton(text="Open file")
        open_file_btn.clicked.connect(self.open_mc_file)
        open_recording_btn = QPushButton(text="Open recording")
        open_recording_btn.clicked.connect(self.open_recording)
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
        self.analogs.selectionChanged = self.selected_analog

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

        self.convert_text = {
                "intro": """""",
                "raw_data": "",
                "digital": "",
                }

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
        if globals.CURRENT_MC_FILE is not None:
            self.explorer = MCExplorer(globals.CURRENT_MC_FILE)
            # read recordings
            for recording in self.explorer.list_recordings():
                item = QListWidgetItem()
                item.setText(recording[1])
                item.setData(1, recording[0])
                self.recordings.addItem(item)

    def selected_recording(self, new_item, old_item):
        index = new_item.indexes()[0].data(1)
        self.recording_info.setText(self.explorer.recording_info(index))
        self.recording_index = index

    def open_recording(self):
        currentRecording = self.recordings.currentItem()
        if currentRecording is not None:
            recording_index = currentRecording.data(1)
            # read analogs
            for analog in self.explorer.list_analogs(recording_index):
                item = QListWidgetItem()
                item.setText(analog[1])
                item.setData(1, analog[0])
                self.analogs.addItem(item)

    def selected_analog(self, new_item, old_item):
       index = new_item.indexes()[0].data(1)
       self.analog_info.setText(self.explorer.analog_info(self.recording_index, index))
