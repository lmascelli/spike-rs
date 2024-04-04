use hdf5_rs::types::{AttributeFillable, AttrOpener, Group};

pub struct H5Analog {
    path: String,
    label: String,
    analog_group: Group,
}

impl H5Analog {
    pub fn open(group: Group) -> Result<Self, String> {
        let label = group.open_attr("Label")?;
        Ok(Self {
            path: group.get_path(),
            label: String::from_attribute(&label)?,
            analog_group: group,
        })
    }
}

impl std::fmt::Display for H5Analog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "H5Analog:")?;
        writeln!(f, "  path: {}", self.path)?;
        writeln!(f, "  label: {}", self.label)?;
        Ok(())
    }
}
