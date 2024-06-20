use spike_rs::operations::math::{convolve, train};
use spike_rs::plot::ToPyList;
use std::collections::HashMap;

fn t1() {
    let n = 100;
    let f = 0.01f32;
    let v1: Vec<f32> = (0..n)
        .map(|x| (x as f32 * 2. * std::f32::consts::PI * f).sin())
        .collect();
    let v2 = train(5, 200, 100);
    let v3 = convolve(&v1[..], &v2[..]);
    let mut dict = HashMap::new();
    dict.insert("v1".to_string(), v1);
    dict.insert("v2".to_string(), v2);
    dict.insert("v3".to_string(), v3);
    dict.to_py_list("target/t1.py");
}

pub fn main() {
    t1();
}
