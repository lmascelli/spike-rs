#[path = "h5recording.rs"]
mod h5recording;
use h5recording::H5Recording;
// pub use h5recording::{CInfoChannel, info_channel_type};

use hdf5_rs::types::{AttributeFillable, AttrOpener, File, FileOpenAccess, Group, GroupOpener};
use spike_rs::core::types::Phase;

pub struct H5Content {
    recordings: Vec<H5Recording>,
    path: String,
    _data_group: Group,
    date: String,
}

impl H5Content {
    pub fn open(filename: &str) -> Result<Self, String> {
        let data_group = File::open(filename, FileOpenAccess::ReadOnly)?.open_group("Data")?;
        let date = String::from_attribute(&data_group.open_attr("Date")?)?;
        let mut recordings = vec![];
        for recording in data_group.list_groups() {
            if recording.starts_with("Recording_") {
                recordings.push(H5Recording::open(data_group.open_group(&recording)?)?);
            }
        }
        Ok(Self {
            path: filename.to_string(),
            _data_group: data_group,
            date,
            recordings,
        })
    }

    pub fn list_recordings(&self) -> Vec<(usize, String)> {
        let mut ret = vec![];
        for (i, recording) in self.recordings.iter().enumerate() {
            ret.push((i, recording.get_path()));
        }
        ret
    }

    pub fn get_recording(&self, index: usize) -> Result<&H5Recording, String> {
        if index < self.recordings.len() {
            Ok(&self.recordings[index])
        } else {
            Err(format!("H5Content::get_recording: H5Content {} index {} out of bounds",
                        self.path, index))
        }
    }

    pub fn convert_phase(&self,
                         recording_index: usize,
                         raw_data_index: usize,
                         digital_index: Option<usize>,
                         event_index: Option<usize>,
                         ) -> Result<Phase, String> {
        let mut ret = Phase::default();
        if self.recordings.len() <= recording_index {
            return Err(format!("H5Content::convert_phase: recoding_index {} out of bounds",
                               recording_index));
        } else {
            let recording = &self.recordings[recording_index];
            let raw_data = recording.get_analog(raw_data_index)?;
            for label in raw_data.get_labels() {
                ret.raw_data.insert(label.clone(), raw_data.get_channel(&label)?);
            }
            if let Some(digital_index) = digital_index {
                let digital = recording.get_analog(digital_index)?;
                let digital_labels = digital.get_labels();
                if digital_labels.len() != 1 {
                    return Err(format!(r#"H5Content::convert_phase: digital stream {}
                    has more than 1 channel"#, digital_index));
                } else {
                    ret.digitals.push(digital.get_channel(&digital_labels[0])?);
                }
            }
            if let Some(event_index) = event_index {
                let event = recording.get_event(event_index)?;
                for intervals in event.samples.values() {
                    ret.el_stim_intervals.push(intervals.to_vec());
                }
            }
        }
        ret.sampling_frequency = 10000f32;
        Ok(ret)
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
