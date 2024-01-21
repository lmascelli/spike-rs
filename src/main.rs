extern crate code_rs;
use code_rs::hdf5::H5File;
fn main() {
    // let filename = "E:/unige/raw data/03-10-2023/34341/hdf5/34341_DIV49_basal_0.h5";
    let filename = "E:/rust/spike-rs/test.h5";
    {
        let file = H5File::create(filename);
    }
}
