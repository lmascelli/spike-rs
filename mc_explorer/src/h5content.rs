#[path = "h5recording.rs"]
mod h5recording;
use h5recording::H5Recording;

use hdf5_rs::types::{AttributeFillable, AttrOpener, File, Group, GroupOpener};

pub struct H5Content {
    path: String,
    data_group: Group,
    date: String,
    recordings: Vec<H5Recording>,
}

impl H5Content {
    pub fn open(filename: &str) -> Result<Self, String> {
        let data_group = File::open(filename, None)?.open_group("Data")?;
        let date = String::from_attribute(&data_group.open_attr("Date")?)?;
        let mut recordings = vec![];
        for recording in data_group.list_groups() {
            if recording.starts_with("Recording_") {
                recordings.push(H5Recording::open(data_group.open_group(&recording)?)?);
            }
        }
        Ok(Self {
            path: filename.to_string(),
            data_group,
            date,
            recordings,
        })
    }
}

impl std::fmt::Display for H5Content {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5Content:")?;
        writeln!(f, "  path: {}", self.path)?;
        writeln!(f, "  date: {}", self.date)?;
        writeln!(f, "  recordings:")?;
        writeln!(f, "\n  **************************************************")?;
        for recording in &self.recordings {
            writeln!(f, "    {recording}")?;
        }
        writeln!(f, "  **************************************************\n")?;
        Ok(())
    }
}
