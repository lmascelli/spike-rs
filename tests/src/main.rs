use native_c::*;
use spike_rs::plot::ToPyList;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "/home/leonardo/Documents/unige/data/test.h5";
    
    spike_c_init();

    let mut phase = Phase::open(filename)?;
    println!("{:?}", phase.events(0));

    spike_c_close();
    Ok(())
}
