extern crate code_rs;
use code_rs::hdf5::{h5converter::H5Content, save_phase};
fn main() {
    let filename = "E:/unige/raw data/03-10-2023/34341/hdf5/34341_DIV49_basal_0.h5";
    let save_filename = "E:/rust/spike-rs/test.h5";
    let content = H5Content::open(filename).unwrap();
    let phase = content.fill_phase(2).unwrap();
    save_phase(&phase, save_filename);
}
