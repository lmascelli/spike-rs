use native_c::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "/home/leonardo/Documents/unige/data/18-07-2024/38927/raw/2024-07-18T15-32-4638927_100E_DIV70_nbasal_0001_E-00155.h5";
    spike_c_init();
    let phase = Phase::open(filename)?;

    println!("{:?}\n{}", phase, phase.datalen());

    spike_c_close();
    Ok(())
}
