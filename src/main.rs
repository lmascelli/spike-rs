extern crate code_rs;
use code_rs::hdf5::h5converter::H5Content;

fn main() {
    let filename = "E:/unige/raw data/03-10-2023/34341/hdf5/34341_DIV49_basal_0.h5";
    {
        let content = H5Content::open(filename).unwrap();
        for (key, _value) in &content.analogs[2].labels_dict {
            content.analogs[2].get_channel_data(key);
        }
    }
}
