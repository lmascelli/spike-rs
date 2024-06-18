use std::collections::HashMap;

use hdf5_rs::types::{
    AttrOpener, AttributeFillable, DatasetFillable, DatasetOwner, Group,
};
use hdf5_rs::error::Error;

pub struct H5Event {
    _event_group: Group,
    path: String,
    label: String,
    pub samples: HashMap<String, Vec<u64>>,
}

impl H5Event {
    pub fn open(group: Group) -> Result<Self, Error> {
        let label = group.open_attr("Label")?;
        let path = group.get_path();
        let mut samples = HashMap::new();

        for ds_name in group.list_datasets() {
            if ds_name.starts_with("EventEntity_") {
                let ds = group.get_dataset(&ds_name)?;
                let times = u64::from_dataset_row(&ds, 0)?;
                samples.insert(ds_name, times);
            }
        }

        Ok(Self {
            _event_group: group,
            path,
            label: String::from_attribute(&label)?,
            samples,
        })
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }
}

impl std::fmt::Display for H5Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5Event")?;
        writeln!(f, "Path: {}", self.path)?;
        writeln!(f, "Label: {}", self.label)?;
        for (label, events) in &self.samples {
            writeln!(f, "  {} -> n°events: {}", label, events.len())?;
        }
        Ok(())
    }
}
