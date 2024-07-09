use hdf5_rs::{
    init,
    types::{
        CreateDataSetOptions, DataSetWriter, DataSpace, DataSpaceType,
        DatasetOwner, File, FileOpenAccess, IntoDataType,
    },
};
use spike_h5::{PhaseH5, SpikeH5Error};
use spike_rs::plot::ToPyList;
use spike_rs::types::PhaseHandler;

fn hdf5_test() -> Result<(), SpikeH5Error> {
    let filename = "test2.h5";
    init()?;
    let file = File::open(filename, FileOpenAccess::ReadWrite)?;
    let data = [5];
    let dataset = file.get_dataset("Test")?;
    // let dataset = file.create_dataset(CreateDataSetOptions {
    //     name: "Test",
    //     create_plist: None,
    //     access_plist: None,
    //     link_plist: None,
    //     dtype: i32::into_datatype()?,
    //     dspace: DataSpace::create_dataspace(DataSpaceType::Simple, &[1, data.len() as u64])?,
    // })?;
    data[..].as_ref().to_dataset_row(&dataset, 0, Some(1), None)?;
    Ok(())
}

fn phaseh5_test0() -> Result<(), SpikeH5Error> {
    let filename = r#"/home/leonardo/Documents/unige/data/test.h5"#;
    let mut phase = PhaseH5::open(filename)?;
    let data = [0f32; 1000000];
    phase.set_raw_data("E-00155 21", Some(5000), &data)?;
    Ok(())
}

fn phaseh5_test1() -> Result<(), SpikeH5Error> {
    let filename = r#"/home/leonardo/Documents/unige/data/test.h5"#;
    let phase = PhaseH5::open(filename)?;
    let data = phase.raw_data("E-00155 21", None, None)?;
    data[..].as_ref().to_py_list("test.py");
    Ok(())
}

pub fn main() {
    //println!("{:?}", hdf5_test());
    println!("{:?}", phaseh5_test0());
    println!("{:?}", phaseh5_test1());
}
