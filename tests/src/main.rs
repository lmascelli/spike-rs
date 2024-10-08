use native_c::*;
use spike_rs::plot::ToPyList;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "/home/leonardo/Documents/unige/data/test.h5";
    
    spike_c_init();

    let mut phase = Phase::open(filename)?;
    //let new_data = vec![0f32; 600000];
    //phase.set_raw_data(&phase.labels()[0], new_data[..].iter().map(|x| *x).collect(), Some(600000));
    let data = phase.raw_data(&phase.labels()[0], None, None);
    data[..].as_ref().to_py_list("a.py");

    spike_c_close();
    Ok(())
}
