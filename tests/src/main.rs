//use spike_h5::{PhaseH5, PhaseH5Error};
//use spike_rs::operations::math::{convolve, train};
//use spike_rs::plot::ToPyList;
//use std::collections::HashMap;
use hdf5_rs::{
    error::Error,
    types::{FileOpenAccess, GroupOpener},
    Hdf5,
};

//#[allow(unused)]
//fn filter_test() {
//    let n = 10;
//    let f = 0.01f32;
//    let v1: Vec<f32> = (0..n)
//        .map(|x| (x as f32 * 2. * std::f32::consts::PI * f).sin())
//        .collect();
//
//    let v2 = train(5, 70, 0);
//    let v3 = convolve(&v1[..], &v2[..]);
//    let mut dict = HashMap::new();
//    dict.insert("v1".to_string(), v1);
//    dict.insert("v2".to_string(), v2);
//    dict.insert("v3".to_string(), v3);
//    dict.to_py_list("target/t1.py");
//}
//
//#[allow(unused)]
//fn phaseh5_test() -> Result<(), PhaseH5Error> {
//    let filename = "/home/leonardo/Documents/unige/data/12-04-2024/38936_DIV77/raw/04_StimEl.h5";
//    println!("{}", PhaseH5::open(filename)?);
//    Ok(())
//}

fn hdf5_test() -> Result<(), Error> {
    let hdf5 = Hdf5::init()?;
    let file = hdf5.open_file("/home/leonardo/Documents/unige/data/12-04-2024/38936_DIV77/raw/04_StimEl.h5", FileOpenAccess::ReadOnly)?;
    let data = file.open_group("Data/Recording_0")?;
    println!("{data}");
    Ok(())
}

pub fn main() {
    //filter_test();
    // let _ = phaseh5_test();
    let res = hdf5_test();
    println!("{:?}", res);
}
