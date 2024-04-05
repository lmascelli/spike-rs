use hdf5_rs::types::{AttributeFillable, AttrOpener, Group, GroupOpener};
#[path = "h5analog.rs"]
mod h5analog;
use h5analog::H5Analog;

pub struct H5Recording {
    path: String,
    duration: i64, 
    recording_group: Group,
    analogs: Vec<H5Analog>,
}

impl H5Recording {
    pub fn open(group: Group) -> Result<Self, String> {
        let duration = group.open_attr("Duration")?;
        let mut analogs = vec![];

        if let Ok(group) = group.open_group("AnalogStream") {
            for analog in group.list_groups() {
                if analog.starts_with("Stream_") {
                    analogs.push(H5Analog::open(group.open_group(&analog)?)?);
                }
            }
        }

        Ok(Self {
            path: group.get_path(),
            duration: i64::from_attribute(&duration)?,
            recording_group: group,
            analogs,
        })
    }
}

impl std::fmt::Display for H5Recording {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5Recording:")?;
        writeln!(f, "  path: {}", self.path)?;
        writeln!(f, "  duration: {}", self.duration)?;
        writeln!(f, "  analog streams:")?;
        writeln!(f, "\n  ++++++++++++++++++++++++++++++")?;
        for analog in &self.analogs {
            writeln!(f, "    {analog}")?;
        }
        writeln!(f, "  ++++++++++++++++++++++++++++++\n")?;
        Ok(())
    }
}
