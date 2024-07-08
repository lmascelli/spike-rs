use hdf5_rs::{init, types::{CreateDataSetOptions, DataSpace, DatasetOwner, DataSetWriter, DataSpaceType, File, FileOpenAccess, IntoDataType}};
use spike_h5::{PhaseH5, SpikeH5Error};
use spike_rs::types::PhaseHandler;
use spike_rs::plot::ToPyList;

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

pub fn main() {
    println!("{:?}", hdf5_test());
}
