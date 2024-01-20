use std::fmt::{Display, Error, Formatter};

pub enum SignalUnit {
    Unknown,
    Volt,
}

pub struct Signal {
    pub data: Vec<f32>,
    pub sampling_frequency: f32,
    pub unit: SignalUnit,
    pub scale: f32,
}

impl Signal {
    pub fn new() -> Signal {
        Signal {
            data: Vec::new(),
            sampling_frequency: 1f32,
            unit: SignalUnit::Unknown,
            scale: 1f32,
        }
    }
}

impl Display for Signal {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), Error> { 
        print!("( ");
        for d in &self.data {
            print!("{} ", d);
        }
        println!(")");
        Ok(())
    }
}
