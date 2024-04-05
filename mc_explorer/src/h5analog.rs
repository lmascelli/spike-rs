use std::collections::HashMap;
use hdf5_rs::types::{AttributeFillable, AttrOpener, DatasetOwner, Group};
use hdf5_rs::cchar_to_string;
use hdf5_rs::h5sys::CStr;

mod info_channel;
use info_channel::{CInfoChannel, info_channel_type};

pub struct H5Analog {
    path: String,
    label: String,
    channels: HashMap<String, Option<Vec<f32>>>,
    analog_group: Group,
}

impl H5Analog {
    pub fn open(group: Group) -> Result<Self, String> {
        let label = group.open_attr("Label")?;
        let infochannel_ds = group.get_dataset("InfoChannel")?;
        let dims = infochannel_ds.get_dataspace()?.get_dims();
        assert!(dims.len() == 1, "InfoChannel dataset should be a vector");
        let mut data = vec![CInfoChannel::default(); dims[0]];

        let info_channels = infochannel_ds.fill_memory(info_channel_type(), &mut data)?;

        for info_channel in &data {
            println!("{}", cchar_to_string!(info_channel.label));
        }
        
        Ok(Self {
            path: group.get_path(),
            label: String::from_attribute(&label)?,
            channels: HashMap::new(),
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
