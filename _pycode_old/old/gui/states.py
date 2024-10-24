from . import globals

###############################################################################
#
#                                 GUI STATES
#.states
###############################################################################

def state_started():
    globals.ROOT.tree.setVisible(False)
    globals.ROOT.controls.open_phase_button.setEnabled(False)
    globals.ROOT.controls.compute_peak_trains_button.setEnabled(False)
    globals.ROOT.controls.convert_phase_button.setEnabled(False)
    globals.ROOT.controls.create_interval_button.setEnabled(False)
    globals.ROOT.controls.plot_signal_button.setEnabled(False)
    globals.ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    globals.ROOT.controls.plot_rasterplot_button.setEnabled(False)
    globals.ROOT.controls.plot_with_intervals_cb.setEnabled(False)
    globals.ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    globals.ROOT.controls.clear_peaks_over_threshold_button.setEnabled(False)
    viewer_widget = globals.ROOT.viewer.widgets['None']
    globals.ROOT.viewer.setCurrentIndex(viewer_widget[0])

def state_inspect_recordings_folder():
    globals.ROOT.tree.setVisible(True)
    globals.ROOT.controls.open_phase_button.setEnabled(False)
    globals.ROOT.controls.compute_peak_trains_button.setEnabled(False)
    globals.ROOT.controls.plot_rasterplot_button.setEnabled(False)
    globals.ROOT.controls.plot_with_intervals_cb.setEnabled(False)
    globals.ROOT.controls.convert_phase_button.setEnabled(False)
    globals.ROOT.controls.create_interval_button.setEnabled(False)
    globals.ROOT.controls.plot_signal_button.setEnabled(False)
    globals.ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    globals.ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    globals.ROOT.controls.clear_peaks_over_threshold_button.setEnabled(False)
    viewer_widget = globals.ROOT.viewer.widgets[ 'PhaseInfo' ]
    globals.ROOT.viewer.setCurrentIndex(viewer_widget[0])

def state_inspect_recordings_folder_phase_selected():
    # globals.ROOT.tree.setVisible(True)                # not managed here
    globals.ROOT.controls.open_phase_button.setEnabled(True)
    globals.ROOT.controls.convert_phase_button.setEnabled(False)
    globals.ROOT.controls.plot_rasterplot_button.setEnabled(False)
    globals.ROOT.controls.plot_with_intervals_cb.setEnabled(False)
    globals.ROOT.controls.compute_peak_trains_button.setEnabled(False)
    globals.ROOT.controls.plot_signal_button.setEnabled(False)
    globals.ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    globals.ROOT.controls.create_interval_button.setEnabled(False)
    globals.ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    globals.ROOT.controls.clear_peaks_over_threshold_button.setEnabled(False)

def state_inspect_phase():
    # globals.ROOT.tree.setVisible(True)                # not managed here
    globals.ROOT.controls.open_phase_button.setEnabled(True)
    globals.ROOT.controls.compute_peak_trains_button.setEnabled(True)
    globals.ROOT.controls.convert_phase_button.setEnabled(True)
    globals.ROOT.controls.create_interval_button.setEnabled(False)
    globals.ROOT.controls.plot_signal_button.setEnabled(False)
    globals.ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    globals.ROOT.controls.plot_rasterplot_button.setEnabled(True)
    globals.ROOT.controls.plot_with_intervals_cb.setEnabled(True)
    globals.ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    globals.ROOT.controls.clear_peaks_over_threshold_button.setEnabled(True)
    phase_info = globals.ROOT.viewer.widgets['PhaseView']
    globals.ROOT.viewer.setCurrentIndex(phase_info[0])
    phase_info[1].update_data()

def state_update_peaks():
    globals.ROOT.controls.open_phase_button.setEnabled(True)
    globals.ROOT.controls.compute_peak_trains_button.setEnabled(False)
    globals.ROOT.controls.convert_phase_button.setEnabled(True)
    globals.ROOT.controls.plot_signal_button.setEnabled(False)
    globals.ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    globals.ROOT.controls.clear_peaks_over_threshold_button.setEnabled(True)
    globals.ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    phase_info = globals.ROOT.viewer.widgets['PhaseView']
    globals.ROOT.viewer.setCurrentIndex(phase_info[0])
    phase_info[1].update_peaks()

def state_selected_signal():
    globals.ROOT.controls.plot_signal_button.setEnabled(True)
    globals.ROOT.controls.plot_with_peaks_cb.setEnabled(True)
    globals.ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    globals.ROOT.controls.create_interval_button.setEnabled(False)
    if globals.CURRENT_SELECTED_SIGNAL is not None and globals.CURRENT_SELECTED_SIGNAL[0] == 'digital':
        switch_state('SELECTED_DIGITAL')

def state_selected_peak_train():
    globals.ROOT.controls.plot_signal_button.setEnabled(False)
    globals.ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    globals.ROOT.controls.plot_peaks_histogram_button.setEnabled(True)
    globals.ROOT.controls.create_interval_button.setEnabled(False)

def state_selected_digital():
    globals.ROOT.controls.plot_signal_button.setEnabled(True)
    globals.ROOT.controls.plot_with_peaks_cb.setEnabled(False)
    globals.ROOT.controls.plot_peaks_histogram_button.setEnabled(False)
    globals.ROOT.controls.create_interval_button.setEnabled(True)

def state_peak_detection_done():
    phase_info = globals.ROOT.viewer.widgets['PhaseView']
    globals.ROOT.viewer.setCurrentIndex(phase_info[0])
    phase_info[1].update_peaks()

def state_explorer_enter():
    explorer = globals.ROOT.viewer.widgets['Explorer']
    globals.ROOT.viewer.setCurrentIndex(explorer[0])

def state_explorer_set_recording():
    explorer = globals.ROOT.viewer.widgets['Explorer']



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
    'EXPLORER_ENTER': state_explorer_enter,
    'EXPLORER_SET_RECORDING': state_explorer_set_recording,
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
