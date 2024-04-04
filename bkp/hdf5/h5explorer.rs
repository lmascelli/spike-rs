use crate::hdf5::sys::*;
use crate::hdf5::utils::*;

mod h5parse;
use h5parse::_inspect_recordings;

#[derive(Debug)]
pub struct H5Analog {
    pub path: String,
    pub label: String,
    pub shape: Vec<usize>,
    pub sampling_frequency: f32,
}

impl H5Analog {
    pub fn new(path: String) -> Self {
        Self {
            path,
            label: "Undefined".to_string(),
            shape: vec![],
            sampling_frequency: 0f32,
        }
    }
}

#[derive(Debug)]
pub struct H5Event {}

#[derive(Debug)]
pub struct H5Recording {
    pub path: String,
    pub duration: i64,
    pub analogs: Vec<H5Analog>,
    pub events: Vec<H5Event>,
}

impl H5Recording {
    pub fn new(path: String) -> Self {
        H5Recording {
            path,
            duration: 0,
            analogs: vec![],
            events: vec![],
        }
    }
}


#[derive(Debug, Default)]
pub struct H5Content {
    date: String,
    pub recordings: Vec<H5Recording>,
    // private file fields
    fid: i64,
}

impl H5Content {
    pub fn from_file(filename: &str) -> Result<Self, String> {
        let mut ret = Self::default();
        let data_id;

        // open the file for read and check if the opeation succeded
        {
            let cfilename = CString::new(filename);
            if cfilename.is_err() {
                return Err(format!(
                    "H5Content::from_file : invalid filename {}", filename));
            } 

            ret.fid = unsafe { H5Fopen(cfilename.unwrap().as_c_str().as_ptr(),
                H5F_ACC_RDONLY, H5P_DEFAULT) };
            if ret.fid <= 0 {
                return Err(format!("convert_mc_h5_file: failed opening {}",
                filename));
            }
        }

        // retrieve the Date attribute
        {
            data_id = unsafe {H5Gopen2(ret.fid, str_to_cchar("/Data\0"), H5P_DEFAULT)};
            let date = get_attr_str(data_id, "Date\0", 13);
            ret.date = date.expect("H5Content::from_file: Failed to retrieve recordings date");
        }

        // cicle over Recording groups
        unsafe {
            H5Literate2(data_id,
                        H5_index_t_H5_INDEX_NAME,
                        H5_iter_order_t_H5_ITER_INC,
                        null_mut(),
                        Some(_inspect_recordings),
                        &ret as *const H5Content as *mut c_void);
        }

        // closing opened ids
        unsafe {
            H5Gclose(data_id);
        }

        Ok(ret)
    }
}

impl std::fmt::Display for H5Content {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "Date: {}", self.date)?;
        for recording in &self.recordings {
            writeln!(fmt, "Recording:")?;
            writeln!(fmt, "  {}", recording.path)?;
            writeln!(fmt, "  Duration: {}:", recording.duration)?;
            for (i, ref analog) in recording.analogs.iter().enumerate() {
                writeln!(fmt, "  Analog:")?;
                writeln!(fmt, "    Index: {}", i)?;
                writeln!(fmt, "    Path:  {}", analog.path)?;
                writeln!(fmt, "    Label: {}", analog.label)?;
                writeln!(fmt, "    Shape: {:?}", analog.shape)?;
            }
        }
        
        Ok(())
    }
}

impl Drop for H5Content {
    fn drop(&mut self) {
        unsafe {
            H5Fclose(self.fid);
        }
    }
}
