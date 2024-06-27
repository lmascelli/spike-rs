use hdf5_rs::{
    error::Error,
    types::{DatasetFillable, DatasetOwner, FileOpenAccess, GroupOpener},
    Hdf5,
};
use spike_rs::plot::ToPyList;

extern "C" fn convert(
    flags: u32,
    cd_nelmts: usize,
    cd_values: *const u32,
    nbytes: usize,
    buf_size: *mut usize,
    buf: *mut *mut core::ffi::c_void,
) -> usize {
    0
}

fn hdf5_test() -> Result<(), Error> {
    let hdf5 = Hdf5::init()?;
    let file = hdf5.open_file(
        "/home/leonardo/Documents/unige/data/12-04-2024/38936_DIV77/raw/04_StimEl.h5",
        FileOpenAccess::ReadOnly)?;
    let data = file.open_group("Data/Recording_0/AnalogStream/Stream_0")?;
    let dataset = data.get_dataset("ChannelData")?;
    println!("{:?}", dataset.get_chunk()?);
    let data: Vec<f32> = i32::from_dataset_row(&dataset, 0, None)?
        .iter()
        .map(|x| *x as f32)
        .collect();
    (&data[..]).to_py_list("test.py");
    Ok(())
}

pub fn main() {
    let res = hdf5_test();
    println!("{:?}", res);
}
