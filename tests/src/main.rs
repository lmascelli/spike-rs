use native_c::*;
use spike_rs::{
    types::PhaseHandler,
    plot::ToPyList
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "/home/leonardo/Documents/unige/data/test.h5";
    
    match spike_c_init() {
        Ok(()) => {
            println!("OK");
            let mut phase = Phase::open(filename)?;

            let labels = phase.labels();
            for label in labels {
                println!("{label}");
            }

            println!("{}", phase.datalen());
            println!("{}", phase.sampling_frequency());

            spike_c_close();
            Ok(())
        },
        Err(err) => {
            println!("{err:?}");
            Ok(())
        }
    }
}
