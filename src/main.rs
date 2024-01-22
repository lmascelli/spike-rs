extern crate code_rs;
use code_rs::hdf5::{h5converter::H5Content, save_phase, H5File, IntoH5Field};
fn main() {
    let filename = "E:/unige/raw data/03-10-2023/34341/hdf5/34341_DIV49_basal_0.h5";
    let save_filename = "E:/rust/spike-rs/test.h5";
    let content = H5Content::open(filename).unwrap();
    let mut phase = content.fill_phase(2).unwrap();

    let mut savefile = H5File::create(save_filename).unwrap();
    let file_handle = savefile.add_struct("Data").unwrap();
    let label = "46";
    let raw_data = &phase.raw_datas[label].data[..];
    raw_data.into_h5field(file_handle.id(), "raw_data");
    if let Some(_) = phase.compute_peak_train(label) {
        let peak_train = &phase.peak_trains[label][..];
        peak_train.into_h5field(file_handle.id(), "peak_train");
    }
}
