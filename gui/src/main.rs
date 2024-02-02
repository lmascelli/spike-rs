extern crate spike_rs;
use spike_rs::hdf5::{convert_mc_h5_file, save_phase, load_phase};

fn main() {
    let savefile = "test.h5";
    match load_phase(savefile) {
        Ok(_) => { println!("OK!"); },
        Err(err) => { println!("{err}"); }
    }
}
