extern crate spike_rs;
use spike_rs::hdf5::{convert_mc_h5_file, save_phase};

fn main() {
    let filename = "E:/unige/raw data/03-10-2023/34341/hdf5/34341_DIV49_basal_9.h5";
    let savefile = "test.h5";
    if let Ok(phase) = convert_mc_h5_file(filename) {
        save_phase(&phase, savefile);
    }
}
