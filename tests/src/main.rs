use spike_rs::operations::math::{convolve, train};
use spike_rs::plot::ToPyList;
use std::collections::HashMap;
use spike_h5::{PhaseH5, PhaseH5Error};

fn filter_test() {
    let n = 10;
    let f = 0.01f32;
    let v1: Vec<f32> = (0..n)
        .map(|x| (x as f32 * 2. * std::f32::consts::PI * f).sin())
        .collect();

    let v2 = train(5, 70, 0);
    let v3 = convolve(&v1[..], &v2[..]);
    let mut dict = HashMap::new();
    dict.insert("v1".to_string(), v1);
    dict.insert("v2".to_string(), v2);
    dict.insert("v3".to_string(), v3);
    dict.to_py_list("target/t1.py");
}

fn phaseh5_test() -> Result<(), PhaseH5Error> {
    let filename = "/home/leonardo/Documents/unige/data/12-04-2024/38936_DIV77/raw/04_StimEl.h5";
    PhaseH5::open(filename)?;
    Ok(())
}

pub fn main() {
    //filter_test();
    let _ = phaseh5_test();
}
