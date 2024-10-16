use native_c::*;
use spike_rs::{
    types::PhaseHandler,
    plot::ToPyList
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "C:/Users/Leonardo/Desktop/18-07-2024/38927/raw/38927/2024-07-18T15-36-4438927_100E_DIV70_Stim70_0002_E-00155.h5";
    
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
