use spike_rs::hdf5::{load_phase, save_phase};
use spike_rs::core::operations::compute_threshold;

fn main() {
    let filename = "test2.h5";
    let mut phase = load_phase(filename).expect("Failed to load phase");
}
