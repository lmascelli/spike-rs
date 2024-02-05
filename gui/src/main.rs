use spike_rs::hdf5::{load_phase, save_phase};
use spike_rs::core::operations::compute_threshold;

fn main() {
    let filename = "test.h5";
    let mut phase = load_phase(filename).expect("Failed to load phase");
    phase.compute_all_peak_trains();
    let _ = save_phase(&phase, filename);
}
