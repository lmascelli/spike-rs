use native_c::*;
use spike_rs::{
    types::PhaseHandler,
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "/home/leonardo/Documents/unige/data/18-07-2024/38927/raw/2024-07-18T16-39-2038927_100E_DIV70_Stim70_00018_E-00155.h5";
    
    match spike_c_init() {
        Ok(()) => {
            println!("OK");
            let phase = Phase::open(filename)?;

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
