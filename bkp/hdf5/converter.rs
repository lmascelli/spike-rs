////////////////////////////////////////////////////////////////////////////////
///
///                          MultiChannel Converted
///
////////////////////////////////////////////////////////////////////////////////

use crate::hdf5::sys::*;

extern "C" fn _parse_analog_stream(group: i64,
                                   name: *const i8,
                                   _info: *const H5L_info2_t,
                                   data: *mut c_void) -> i32 {
    let phase = unsafe { &mut*(data as *mut Phase) };
    let inner_group = unsafe { H5Gopen2(group, name, H5P_DEFAULT) };
    let mut is_digital = false;
    let info_channel_dataset;
    let mut info_channels = Vec::new();
    unsafe {
        info_channel_dataset = H5Dopen2(inner_group,
                                        CStr::from_bytes_with_nul("InfoChannel\0"
                                                                  .as_bytes()).unwrap().as_ptr(),
                                        H5P_DEFAULT);
        let info_channel_dataspace = H5Dget_space(info_channel_dataset);

        // get the number of channels
        let mut dims = 0u64;
        H5Sget_simple_extent_dims(info_channel_dataspace, &mut dims as *mut u64, null_mut());
        if dims == 1 {
	    is_digital = true;
        }

        // prepare memory for holding infochannels metadata
        info_channels.resize(dims as usize, CInfoChannel::default());
        let info_channel_memory_datatype = load_info_type();

        // read the metadatas
        H5Dread(info_channel_dataset, info_channel_memory_datatype, H5S_ALL, H5S_ALL,
                H5P_DEFAULT, info_channels.as_ptr() as _);

        H5Tclose(info_channel_memory_datatype);
        H5Sclose(info_channel_dataspace);
        H5Dclose(info_channel_dataset);
    }
    // 
    let channel_data_dataset;
    unsafe {
        channel_data_dataset = H5Dopen2(inner_group,
                                        CStr::from_bytes_with_nul("ChannelData\0"
                                                                  .as_bytes()).unwrap().as_ptr(),
                                        H5P_DEFAULT);

        let channel_data_dataspace = H5Dget_space(channel_data_dataset);
        let ndims = H5Sget_simple_extent_ndims(channel_data_dataspace);
        let dims = vec![0; ndims as usize];
        H5Sget_simple_extent_dims(channel_data_dataspace, dims.as_ptr().cast_mut(), null_mut());

        // let n_channels = dims[0];
        let n_samples = dims[1];

        for info_channel in info_channels {
	    // get channel label
	    let label = CStr::from_ptr(info_channel.label).to_str().unwrap();

	    let sampling_frequency = 1e4f32;

	    // get channel row in dataspace
	    let row_index = info_channel.row_index as u64;

	    // get channel adc offset
	    let adc_offset = info_channel.ad_zero as f32;

	    // get channel conversion factor
	    let conversion_factor = info_channel.conversion_factor as f32 * f32::powf(10f32, info_channel.exponent as f32);

	    // set the dataspace slub
	    let starting_point = [row_index, 0];
	    let length_data_to_read = [1u64, n_samples];
	    H5Sselect_hyperslab(channel_data_dataspace, H5S_SELECT_SET, starting_point.as_ptr(),
				null(), length_data_to_read.as_ptr(), null());

	    // allocate the memory;
	    let data_to_be_converted = vec![0i32; n_samples as usize];

	    // create the memory dataspace
	    let memory_size = [dims[1]];
	    let channel_data_memory_dataspace = H5Screate_simple(1, memory_size.as_ptr(), null_mut());

	    // read the data
	    H5Dread(channel_data_dataset, H5T_NATIVE_INT_g, channel_data_memory_dataspace,
		    channel_data_dataspace, H5P_DEFAULT, data_to_be_converted.as_ptr() as _);

	    // convert the data
	    let mut converted_data = vec![0f32; n_samples as usize];

	    for (i, value) in data_to_be_converted.iter().enumerate() {
                converted_data[i] = (*value as f32 - adc_offset) * conversion_factor;
	    }

	    if is_digital {
                phase.digitals.push(converted_data);
	    } else {
                phase.raw_data.insert(label.to_string(), converted_data);
	    }

	    phase.sampling_frequency = sampling_frequency;

        }

        H5Sclose(channel_data_dataspace);
        H5Dclose(channel_data_dataset);
    }
    0
}

extern "C" fn _parse_event_stream(group: i64,
                                  name: *const i8,
                                  _info: *const H5L_info2_t,
                                  data: *mut c_void) -> i32 {
    let phase = unsafe { &mut*(data as *mut Phase) };
    let name_str = unsafe { CStr::from_ptr(name).to_str().unwrap() };
    if name_str.starts_with("EventEntity_") {
        unsafe {
	    // get the dataset and dataspace of the group
	    let events_dataset = H5Dopen2(group, name, H5P_DEFAULT);
	    let events_dataspace = H5Dget_space(events_dataset);

	    // check the rank of the dataset
	    let events_dims = H5Sget_simple_extent_ndims(events_dataspace);
	    assert!(events_dims == 2, "convert_mc_h5_file: error in converting the EventStream. Wrong rank of the EventEntity dataset");

	    // get the number of samples
	    let dims = vec![0; events_dims as usize];
	    H5Sget_simple_extent_dims(events_dataspace, dims.as_ptr().cast_mut(), null_mut());
	    let n_samples = dims[1] as usize;

	    // set the hyperslab of the data to be read
	    let starting_point = [0, 0];
	    let length_data_to_read = [1, n_samples as u64];
	    H5Sselect_hyperslab(events_dataspace, H5S_SELECT_SET, starting_point.as_ptr(),
                                null(), length_data_to_read.as_ptr(), null());

	    // create the memory dataspace
	    //  and allocate memory for reading the samples
	    let mut events = vec![0u64; n_samples];
	    let memory_size = [n_samples as u64];
	    let events_memory_dataspace = H5Screate_simple(1, memory_size.as_ptr(), null_mut());

	    // read the data
	    H5Dread(events_dataset, H5T_NATIVE_LLONG_g, events_memory_dataspace, events_dataspace,
		    H5P_DEFAULT, events.as_ptr() as _);

	    // rescale the events from microseconds to bins. this requires the sampling frequency
	    // to be correctly set at this time!!!
	    let scaling_factor = phase.sampling_frequency * 1e-6;
	    for event in &mut events {
		*event = (*event as f32 * scaling_factor) as u64;
	    }

	    phase.el_stim_intervals.push(events);
        }
    }
    0
}

pub fn convert_mc_h5_file(filename: &str) -> Result<Phase, String> {

    let ret = Phase::default();

    // Open file and get the Recording_0 group
    if let Ok(cfilename) = CString::new(filename) {
        let fid = unsafe { H5Fopen(cfilename.as_c_str().as_ptr(), H5F_ACC_RDONLY, H5P_DEFAULT) };
        if fid <= 0 {
	    return Err(format!("convert_mc_h5_file: failed opening {}", filename));
        }
        let analogs_id = unsafe { H5Gopen2(fid,
                                           CStr::from_bytes_with_nul("/Data/Recording_0/AnalogStream\0"
								     .as_bytes()).unwrap().as_ptr(),
                                           H5P_DEFAULT) };
        if analogs_id <= 0 {
	    return Err(format!("convert_mc_h5_file: error opening Recording_0 group in file {}", filename));
        }

        // parse the Stream_X channels in the analogs_id
        unsafe {
	    H5Literate2(analogs_id, 
                        H5_index_t_H5_INDEX_NAME,
                        H5_iter_order_t_H5_ITER_INC,
                        null_mut(),
                        Some(_parse_analog_stream),
                        &ret as *const Phase as *mut c_void);
        }

        // parse the Stream_X channel in the EventStream id
        let events_id = unsafe { H5Gopen2(fid,
                                          CStr::from_bytes_with_nul("/Data/Recording_0/EventStream/Stream_0\0"
								    .as_bytes()).unwrap().as_ptr(),
                                          H5P_DEFAULT) };
        if events_id > 0 {
	    unsafe {
                H5Literate2(events_id, 
			    H5_index_t_H5_INDEX_NAME,
			    H5_iter_order_t_H5_ITER_INC,
			    null_mut(),
			    Some(_parse_event_stream),
			    &ret as *const Phase as *mut c_void);
	    }
        }

        unsafe {
	    H5Gclose(analogs_id);
	    H5Fclose(fid);
        }
        Ok(ret)
    } else {
        Err(format!("convert_mc_h5_file: invalid filename {}", filename))
    }
}
