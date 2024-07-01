use hdf5_rs::{init, close};
use spike_h5::{PhaseH5, SpikeH5Error};

fn hdf5_test() -> Result<(), SpikeH5Error> {
    let filename = "/home/leonardo/Documents/unige/data/12-04-2024/38940_DIV77/raw/2024-04-11T14-35-5338940_100E_DIV77_StimEl_0002_E-00155.h5";
    init()?;
    let phase = PhaseH5::open(filename)?;
    println!("Duration: {}", phase.duration);
    close();
    Ok(())
}

pub fn main() {
    println!("{:?}", hdf5_test());
}
