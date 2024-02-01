use std::fmt::{Display, Error, Formatter};

pub struct Signal {
    pub data: Vec<f32>,
    pub sampling_frequency: f32,
}

impl Signal {
    pub fn new(data: Vec<f32>, sampling_frequency: f32) -> Signal {
        Signal {
            data,
            sampling_frequency,
        }
    }
}

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "( ")?;
        for d in &self.data {
            write!(f, "{} ", d)?;
        }
        writeln!(f, ")")?;
        Ok(())
    }
}
