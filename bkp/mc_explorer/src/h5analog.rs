use hdf5_rs::cchar_to_string;
use hdf5_rs::error::Error;
use hdf5_rs::h5sys::CStr;
use hdf5_rs::types::{
    AttrOpener, AttributeFillable, DatasetFillable, DatasetOwner, Group,
};
use std::cell::RefCell;
use std::collections::HashMap;

mod info_channel;
pub use info_channel::{info_channel_type, CInfoChannel};

#[derive(Default)]
struct ChannelInfo {
    index: usize,
    conversion_factor: f32,
    offset: f32,
    exponent: f32,
}

pub struct H5Analog {
    path: String,
    label: String,

    channels_info: HashMap<String, ChannelInfo>,
    channels_data: HashMap<String, RefCell<Option<Vec<f32>>>>,
    analog_group: Group,
}

impl H5Analog {
    pub fn open(group: Group) -> Result<Self, Error> {
        let label = group.open_attr("Label")?;
        let infochannel_ds = group.get_dataset("InfoChannel")?;
        let dims = infochannel_ds.get_dataspace()?.get_dims();
        assert!(dims.len() == 1, "InfoChannel dataset should be a vector");
        let mut data = vec![CInfoChannel::default(); dims[0]];

        infochannel_ds.fill_memory(info_channel_type(), &mut data)?;

        let mut channels_info = HashMap::new();
        let mut channels_data = HashMap::new();
        for info_channel in &data {
            channels_info.insert(
                cchar_to_string!(info_channel.label),
                ChannelInfo {
                    index: info_channel.row_index as usize,
                    conversion_factor: info_channel.conversion_factor as f32,
                    offset: info_channel.ad_zero as f32,
                    exponent: info_channel.exponent as f32,
                },
            );
            channels_data.insert(
                cchar_to_string!(info_channel.label),
                RefCell::new(None),
            );
        }

        Ok(Self {
            path: group.get_path(),
            label: String::from_attribute(&label)?,
            channels_info,
            channels_data,
            analog_group: group,
        })
    }

    pub fn get_dims(&self) -> Result<Vec<usize>, Error> {
        Ok(self
            .analog_group
            .get_dataset("ChannelData")?
            .get_dataspace()?
            .get_dims()
            .to_vec())
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_channel(&self, label: &str) -> Result<Vec<f32>, Error> {
        if self.channels_info.contains_key(label) {
            let mut ret = vec![];
            let channel_data =
                self.channels_data.get(label).unwrap().borrow_mut();
            if let Some(data) = channel_data.as_ref() {
                ret.extend_from_slice(&data[..]);
            } else {
                let channel_info = self.channels_info.get(label).unwrap();
                let channel_data_ds =
                    self.analog_group.get_dataset("ChannelData")?;
                let data = i32::from_dataset_row(
                    &channel_data_ds,
                    channel_info.index,
                )?;
                let offset = channel_info.offset;
                let conversion_factor = channel_info.conversion_factor
                    * 10f32.powf(channel_info.exponent);
                ret.resize(data.len(), 0f32);
                for (i, val) in data.iter().enumerate() {
                    ret[i] = (*val as f32 - offset) * conversion_factor;
                }
            }
            Ok(ret)
        } else {
            Err(Error::group_doesnt_exists(label))
        }
    }

    pub fn get_labels(&self) -> Vec<String> {
        let mut ret = vec![];
        for label in self.channels_info.keys() {
            ret.push(label.clone());
        }
        ret
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
