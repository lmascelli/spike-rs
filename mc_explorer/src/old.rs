////////////////////////////////////////////////////////////////////////////////
///
///                          MultiChannel Converted
///
////////////////////////////////////////////////////////////////////////////////

use spike_rs::core::types::Phase;
use hdf5_rs::h5sys::*;
use crate::h5content::{CInfoChannel, info_channel_type as load_info_type};


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

pub fn save_phase(phase: &Phase, filename: &str) -> Result<(), String> {
    if let Ok(cfilename) = CString::new(filename) {
	let mut sampling_frequency = 0f32;
	let savefile_id = unsafe { H5Fcreate(cfilename.as_c_str().as_ptr(),
					     H5F_ACC_TRUNC,
					     H5P_DEFAULT, H5P_DEFAULT) };
	if savefile_id > 0 {

	    // save digitals
	    let digitals_group_name = "digitals\0".to_string();
	    let digitals_group = unsafe { H5Gcreate2(savefile_id,
						     CStr::from_bytes_with_nul(
							 digitals_group_name
							     .as_bytes())
						     .unwrap().as_ptr(),
						     H5P_DEFAULT,
						     H5P_DEFAULT,
						     H5P_DEFAULT) };
	    if digitals_group > 0 {
                for (i, digital) in phase.digitals.iter().enumerate() {
		    let digital_name = format!("digital_{i}\0");
		    let digital_len = [digital.len() as u64];
		    let digital_dataspace = unsafe {
                        H5Screate_simple(1, digital_len.as_ptr() ,null())
		    };
		    let digital_dataset = unsafe {
			H5Dcreate2(digitals_group,
                                   CStr::from_bytes_with_nul(digital_name
                                                             .as_bytes())
                                   .unwrap().as_ptr(),
                                   H5T_NATIVE_FLOAT_g,
                                   digital_dataspace,
                                   H5P_DEFAULT,
                                   H5P_DEFAULT,
                                   H5P_DEFAULT)
		    };
		    if digital_dataset > 0 {
                        unsafe {
			    H5Dwrite(digital_dataset,
				     H5T_NATIVE_FLOAT_g,
				     digital_dataspace,
				     H5S_ALL,
				     H5P_DEFAULT,
				     digital.as_ptr().cast());
			    H5Dclose(digital_dataset);
                        }
		    } else {
                        return Err(format!(
			    "save_phase: failed to create digital group {}",
			    digital_name));
		    }
                }
	    } else {
                return Err(format!(
		    "save_phase: failed to create digitals group {}",
		    digitals_group_name));
	    }

	    // save electric stimulation timestamps
	    let el_stim_group_name = "electric_stim_events\0".to_string();
	    let el_stim_group = unsafe { H5Gcreate2(savefile_id,
						    CStr::from_bytes_with_nul(
							el_stim_group_name
							    .as_bytes())
						    .unwrap().as_ptr(),
						    H5P_DEFAULT,
						    H5P_DEFAULT,
						    H5P_DEFAULT) };

	    if el_stim_group > 0 {
                for (i, events) in phase.el_stim_intervals.iter().enumerate() {
		    let events_dataset_name = format!("events_{}\0", i);
		    let events_len = [events.len() as u64];
		    let events_dataspace = unsafe {
			H5Screate_simple(1, events_len.as_ptr(), null())
		    };
		    let events_dataset = unsafe {
			H5Dcreate2(el_stim_group,
                                   CStr::from_bytes_with_nul(events_dataset_name
                                                             .as_bytes())
                                   .unwrap().as_ptr(),
                                   H5T_NATIVE_LLONG_g,
                                   events_dataspace,
                                   H5P_DEFAULT,
                                   H5P_DEFAULT,
                                   H5P_DEFAULT)
		    };
		    if events_dataset > 0 {
                        unsafe {
			    H5Dwrite(events_dataset,
				     H5T_NATIVE_LLONG_g,
				     events_dataspace,
				     H5S_ALL,
				     H5P_DEFAULT,
				     events.as_ptr().cast());
			    H5Dclose(events_dataset);
                        }
		    } else {
                        return Err(format!("save_phase: failed to create events dataset {}", events_dataset_name));
		    }
                }
	    } else {
                return Err(format!("save_phase: failed to create electric stimulation group {}", el_stim_group_name));
	    }


	    // save raw_datas
	    let raw_data_name = "raw_data\0".to_string();
	    let raw_data_group = unsafe {H5Gcreate2(savefile_id,
						    CStr::from_bytes_with_nul(raw_data_name
                                                      .as_bytes())
						    .unwrap().as_ptr(),
						    H5P_DEFAULT,
						    H5P_DEFAULT,
						    H5P_DEFAULT) };
	    if raw_data_group > 0 {
                for (label, channel) in &phase.raw_data {
		    sampling_frequency = phase.sampling_frequency;
		    let channel_name = format!("{label}\0");
		    let channel_len = [channel.len() as u64];
		    let channel_dataspace = unsafe {
                        H5Screate_simple(1, channel_len.as_ptr() ,null())
		    };
		    let channel_dataset = unsafe {H5Dcreate2(raw_data_group,
							     CStr::from_bytes_with_nul(channel_name
                                                           .as_bytes())
							     .unwrap().as_ptr(),
							     H5T_NATIVE_FLOAT_g,
							     channel_dataspace,
							     H5P_DEFAULT,
							     H5P_DEFAULT,
							     H5P_DEFAULT)};

		    if channel_dataset > 0 {
                        unsafe { 
			    H5Dwrite(channel_dataset,
				     H5T_NATIVE_FLOAT_g,
				     channel_dataspace,
				     H5S_ALL,
				     H5P_DEFAULT,
				     channel.as_ptr().cast());
			    H5Dclose(channel_dataset);
                        }
		    } else {
                        return Err(format!("save_phase: failed to create raw_data group {}", label));
		    }
                }
                unsafe { H5Gclose(raw_data_group) };
	    } else {
                return Err(format!("save_phase: failed to create raw_data group {}", raw_data_name));
	    }

	    // save the sampling frequency TODO
	    let sampling_frequency_dataspace = unsafe { H5Screate(H5S_class_t_H5S_SCALAR) };
	    let sampling_frequency_dataset = unsafe { H5Dcreate2(savefile_id,
                                                                 CStr::from_bytes_with_nul("sampling_frequency\0".as_bytes())
                                                                 .unwrap().as_ptr(),
                                                                 H5T_NATIVE_FLOAT_g,
                                                                 sampling_frequency_dataspace,
                                                                 H5P_DEFAULT,
                                                                 H5P_DEFAULT,
                                                                 H5P_DEFAULT) };
	    unsafe {
                H5Dwrite(sampling_frequency_dataset,
                         H5T_NATIVE_FLOAT_g,
                         sampling_frequency_dataspace,
                         H5S_ALL,
                         H5P_DEFAULT,
                         (&sampling_frequency as *const f32).cast());
	    }

	    // save peak_trains
	    let peaks_train_name = "peaks_train\0".to_string();
	    let peaks_train_group = unsafe {H5Gcreate2(savefile_id,
						       CStr::from_bytes_with_nul(peaks_train_name
                                                         .as_bytes())
						       .unwrap().as_ptr(),
						       H5P_DEFAULT,
						       H5P_DEFAULT,
						       H5P_DEFAULT) };
	    if peaks_train_group > 0 {
                for (label, (channel_vals, channel_times)) in &phase.peaks_trains {
		    let channel_group_name = format!("{label}\0");
		    let channel_group = unsafe { H5Gcreate2(peaks_train_group,
							    CStr::from_bytes_with_nul(channel_group_name
										      .as_bytes())
							    .unwrap().as_ptr(),
							    H5P_DEFAULT,
							    H5P_DEFAULT,
							    H5P_DEFAULT) };

		    // insert peaks values
		    let channel_len = [channel_times.len() as u64];
		    let channel_values_dataspace = unsafe {
                        H5Screate_simple(1, channel_len.as_ptr() ,null())
		    };
		    let channel_values_name = "values\0".to_string();
		    let channel_values_dataset = unsafe {H5Dcreate2(channel_group,
								    CStr::from_bytes_with_nul(channel_values_name
                                                              .as_bytes())
								    .unwrap().as_ptr(),
								    H5T_NATIVE_FLOAT_g,
								    channel_values_dataspace,
								    H5P_DEFAULT,
								    H5P_DEFAULT,
								    H5P_DEFAULT)};

		    if channel_values_dataset > 0 {
                        unsafe { 
			    H5Dwrite(channel_values_dataset,
				     H5T_NATIVE_FLOAT_g,
				     channel_values_dataspace,
				     H5S_ALL,
				     H5P_DEFAULT,
				     channel_vals.as_ptr().cast());
			    H5Sclose(channel_values_dataspace);
			    H5Dclose(channel_values_dataset);
                        }
		    } else {
                        return Err(format!("save_phase: failed to create peaks_train group {}", label));
		    }

		    // insert peaks times
		    let channel_len = [channel_times.len() as u64];
		    let channel_times_dataspace = unsafe {
                        H5Screate_simple(1, channel_len.as_ptr(), null())
		    };
		    let channel_times_name = "times\0".to_string();
		    let channel_times_dataset = unsafe {H5Dcreate2(channel_group,
                                                                   CStr::from_bytes_with_nul(channel_times_name
                                                                                             .as_bytes())
                                                                   .unwrap().as_ptr(),
                                                                   H5T_NATIVE_ULLONG_g,
                                                                   channel_times_dataspace,
                                                                   H5P_DEFAULT,
                                                                   H5P_DEFAULT,
                                                                   H5P_DEFAULT)};

		    if channel_times_dataset > 0 {
                        unsafe { 
			    H5Dwrite(channel_times_dataset,
				     H5T_NATIVE_ULLONG_g,
				     channel_times_dataspace,
				     H5S_ALL,
				     H5P_DEFAULT,
				     channel_times.as_ptr().cast());
			    H5Sclose(channel_times_dataspace);
			    H5Dclose(channel_times_dataset);
                        }
		    } else {
                        return Err(format!("save_phase: failed to create peaks_train group {}", label));
		    }
		    unsafe { H5Gclose(channel_group) };
                }
                unsafe { H5Gclose(peaks_train_group) };
	    } else {
                return Err(format!("save_phase: failed to create peaks_train group {}", peaks_train_name));
	    }

	    unsafe { H5Fclose(savefile_id) };
	    Ok(())
        } else {
	    Err(format!("save_phase: failed to create file {}", filename))
        }
    } else {
        Err(format!("save_phase: invalid filename {}", filename))
    }
}

////////////////////////////////////////////////////////////////////////////////
///
///                                   Phase Loader
///
////////////////////////////////////////////////////////////////////////////////


extern "C" fn _load_digitals(group: i64,
			     name:   *const i8,
			     _info:  *const H5L_info2_t,
			     data:  *mut c_void,
) -> i32 {
    let phase = unsafe { &mut *(data as *mut Phase) } as &mut Phase;

    // open dataset
    let digital_dataset = unsafe { H5Dopen2(group, name, H5P_DEFAULT) };

    // get dataspace
    let digital_dataspace = unsafe { H5Dget_space(digital_dataset) };

    // allocate memory
    let n_dims = unsafe { H5Sget_simple_extent_ndims(digital_dataspace) } as usize;
    let dims = vec![0; n_dims];
    unsafe {
        H5Sget_simple_extent_dims(digital_dataspace, dims.as_ptr().cast_mut(), null_mut());
    }

    let digital = vec![0f32; dims[0] as usize];

    // create memory dataspace
    let memory_dataspace = unsafe { H5Screate_simple(1, dims.as_ptr().cast(), null()) };

    // read data
    unsafe {
        H5Dread(digital_dataset,
                H5T_NATIVE_FLOAT_g,
                memory_dataspace,
                H5S_ALL,
                H5P_DEFAULT,
                digital.as_ptr() as *mut c_void);
    }

    phase.digitals.push(digital);

    // close opened ids
    unsafe { 
        H5Sclose(memory_dataspace);
        H5Sclose(digital_dataspace);
        H5Dclose(digital_dataset);
    }
    0
}

extern "C" fn _load_raw_datas(group: i64,
			      name:   *const i8,
			      _info:  *const H5L_info2_t,
			      data:  *mut c_void,
) -> i32 {
    let phase = unsafe { &mut *(data as *mut Phase) } as &mut Phase;

    // open dataset
    let raw_data_dataset = unsafe { H5Dopen2(group, name, H5P_DEFAULT) };

    // get dataspace
    let raw_data_dataspace = unsafe { H5Dget_space(raw_data_dataset) };

    // allocate memory
    let n_dims = unsafe { H5Sget_simple_extent_ndims(raw_data_dataspace) } as usize;
    let dims = vec![0; n_dims];
    unsafe {
        H5Sget_simple_extent_dims(raw_data_dataspace,
                                  dims.as_ptr().cast_mut(),
                                  null_mut());
    }

    let raw_data = vec![0f32; dims[0] as usize];

    // create memory dataspace
    let memory_dataspace = unsafe { H5Screate_simple(1, dims.as_ptr().cast(), null()) };

    // read data
    unsafe {
        H5Dread(raw_data_dataset,
                H5T_NATIVE_FLOAT_g,
                memory_dataspace,
                H5S_ALL,
                H5P_DEFAULT,
                raw_data.as_ptr() as *mut c_void);
    }

    phase.raw_data.insert(unsafe { CStr::from_ptr(name).to_str().unwrap().to_string() }, raw_data);

    // close opened ids
    unsafe { 
        H5Sclose(memory_dataspace);
        H5Sclose(raw_data_dataspace);
        H5Dclose(raw_data_dataset);
    }
    0
}

extern "C" fn _load_peaks_trains(group: i64,
                                 name:   *const i8,
                                 _info:  *const H5L_info2_t,
                                 data:  *mut c_void,
) -> i32 {

    let phase = unsafe { &mut *(data as *mut Phase) } as &mut Phase;
    // return variables
    let mut channel_values_data = Vec::new();
    let mut channel_times_data = Vec::new();

    // open channel group
    let channel_group = unsafe { H5Gopen2(group, name, H5P_DEFAULT) };
    if channel_group > 0 {

        // open values dataset
        let channel_values_dataset_name = "values\0";
        let channel_values_dataset = unsafe { H5Dopen2(channel_group,
						       CStr::from_bytes_with_nul(channel_values_dataset_name
										 .as_bytes())
						       .unwrap().as_ptr(),
						       H5P_DEFAULT) };
        if channel_values_dataset > 0 {
	    // get dataspace
	    let channel_values_dataspace = unsafe { H5Dget_space(channel_values_dataset) };
	    let mut dims = 0u64;
	    unsafe {H5Sget_simple_extent_dims(channel_values_dataspace, &mut dims as *mut u64, null_mut())};
	    // allocate memory and create memory dataspace
	    let memory_dataspace = unsafe { H5Screate_simple(1, &dims as *const u64, null()) };
	    channel_values_data.resize(dims as usize, 0f32);
	    // read data
	    unsafe {
                H5Dread(channel_values_dataset,
                        H5T_NATIVE_FLOAT_g,
                        memory_dataspace,
                        H5S_ALL,
                        H5P_DEFAULT,
                        channel_values_data.as_mut_ptr().cast());
                H5Sclose(memory_dataspace);
                H5Sclose(channel_values_dataspace);
                H5Dclose(channel_values_dataset);
	    }
        }
        else {
	    println!("Failed to open values channel {} in peaks trains group",
                     unsafe { CStr::from_ptr(name).to_str().unwrap() });
        }

        // open times dataset
        let channel_times_dataset_name = "times\0";
        let channel_times_dataset = unsafe { H5Dopen2(channel_group,
						      CStr::from_bytes_with_nul(channel_times_dataset_name
										.as_bytes())
						      .unwrap().as_ptr(),
						      H5P_DEFAULT) };
        if channel_times_dataset > 0 {
	    // get dataspace
	    let channel_times_dataspace = unsafe { H5Dget_space(channel_times_dataset) };
	    let mut dims = 0u64;
	    unsafe {H5Sget_simple_extent_dims(channel_times_dataspace, &mut dims as *mut u64, null_mut())};
	    // allocate memory and create memory dataspace
	    let memory_dataspace = unsafe { H5Screate_simple(1, &dims as *const u64, null()) };
	    channel_times_data.resize(dims as usize, 0usize);
	    // read data
	    unsafe {
                H5Dread(channel_times_dataset,
                        H5T_NATIVE_ULLONG_g,
                        memory_dataspace,
                        H5S_ALL,
                        H5P_DEFAULT,
                        channel_times_data.as_mut_ptr().cast());
                H5Sclose(memory_dataspace);
                H5Sclose(channel_times_dataspace);
                H5Dclose(channel_times_dataset);
	    }
        }
        else {
	    println!("Failed to open values channel {} in peaks trains group",
                     unsafe { CStr::from_ptr(name).to_str().unwrap() });
        }

        unsafe { H5Gclose(channel_group) };
        phase.peaks_trains.insert(unsafe { CStr::from_ptr(name).to_str().unwrap().to_string() },
                                  (channel_values_data, channel_times_data));
    } else {
        println!("Failed to open channel {} in peaks trains group",
                 unsafe { CStr::from_ptr(name).to_str().unwrap() });
    }
    0
}

extern "C" fn _load_el_stim_intervals(group: i64,
				      name:   *const i8,
				      _info:  *const H5L_info2_t,
				      data:  *mut c_void,
) -> i32 {
    let phase = unsafe { &mut *(data as *mut Phase) } as &mut Phase;
    
    // open dataset
    let events_dataset = unsafe { H5Dopen2(group, name, H5P_DEFAULT) };

    // get dataspace
    let events_dataspace = unsafe { H5Dget_space(events_dataset) };
    
    // allocate memory
    let n_dims = unsafe { H5Sget_simple_extent_ndims(events_dataspace) } as usize;
    let dims = vec![0; n_dims];
    unsafe {
        H5Sget_simple_extent_dims(events_dataspace,
                                  dims.as_ptr().cast_mut(),
                                  null_mut());
    }

    let events = vec![0u64; dims[0] as usize];

    // create memory dataspace
    let memory_dataspace = unsafe { H5Screate_simple(1, dims.as_ptr().cast(), null()) };
    
    // read data
    unsafe {
        H5Dread(events_dataset,
                H5T_NATIVE_LLONG_g,
                memory_dataspace,
                H5S_ALL,
                H5P_DEFAULT,
                events.as_ptr() as *mut c_void);
    }

    phase.el_stim_intervals.push(events);

    // close opened ids
    unsafe { 
        H5Sclose(memory_dataspace);
        H5Sclose(events_dataspace);
        H5Dclose(events_dataset);
    }
    0
}

pub fn load_phase(filename: &str) -> Result<Phase, String> {
    let mut ret = Phase::default();
    let cfilename = format!("{filename}\0");
    let file_id = unsafe { H5Fopen(CStr::from_bytes_with_nul(cfilename.as_bytes())
                                   .unwrap().as_ptr(),
                                   H5F_ACC_RDONLY,
                                   H5P_DEFAULT) };

    if file_id > 0 {
        // read sampling frequency
        let sampling_frequency_dataset = unsafe { H5Dopen2(file_id, CStr::from_bytes_with_nul("sampling_frequency\0".as_bytes())
                                                           .unwrap().as_ptr(), H5P_DEFAULT) };
        if sampling_frequency_dataset > 0 {
	    unsafe {
                H5Dread(sampling_frequency_dataset,
                        H5T_NATIVE_FLOAT_g,
                        H5S_ALL,
                        H5S_ALL,
                        H5P_DEFAULT,
                        (&mut ret.sampling_frequency as *mut f32).cast());
	    }

	    unsafe { H5Dclose(sampling_frequency_dataset); }
        } else {
	    unsafe { H5Fclose(file_id); }
	    return Err(format!("load_phase: failed opening sampling_frequency dataset in file {}", filename));
        }

        // read digital channels
        let digitals_group = unsafe { H5Gopen2(file_id, CStr::from_bytes_with_nul("digitals\0".as_bytes())
					       .unwrap().as_ptr(), H5P_DEFAULT) };
        if digitals_group > 0 {
	    unsafe {
                H5Literate2(digitals_group, 
			    H5_index_t_H5_INDEX_NAME,
			    H5_iter_order_t_H5_ITER_INC,
			    null_mut(),
			    Some(_load_digitals),
			    &ret as *const Phase as *mut c_void);
                H5Gclose(digitals_group);
	    }
        } else {
	    unsafe { H5Fclose(file_id); }
	    return Err(format!("load_phase: failed opening digitals group in file {}", filename));
        }

        // read electric stimulation channels
        let el_stim_group = unsafe { H5Gopen2(file_id, CStr::from_bytes_with_nul("electric_stim_events\0".as_bytes())
					      .unwrap().as_ptr(), H5P_DEFAULT) };

        if el_stim_group > 0 {
	    unsafe {
                H5Literate2(el_stim_group, 
			    H5_index_t_H5_INDEX_NAME,
			    H5_iter_order_t_H5_ITER_INC,
			    null_mut(),
			    Some(_load_el_stim_intervals),
			    &ret as *const Phase as *mut c_void);
                H5Gclose(el_stim_group);
	    }
        } else {
	    unsafe { H5Fclose(file_id); }
	    return Err(format!("load_phase: failed opening electric stimulation group in file {}", filename));
        }

        // read raw_data channels
        let raw_datas_group = unsafe { H5Gopen2(file_id, CStr::from_bytes_with_nul("raw_data\0".as_bytes())
                                                .unwrap().as_ptr(), H5P_DEFAULT) };
        if raw_datas_group > 0 {
	    unsafe {
                H5Literate2(raw_datas_group, 
			    H5_index_t_H5_INDEX_NAME,
			    H5_iter_order_t_H5_ITER_INC,
			    null_mut(),
			    Some(_load_raw_datas),
			    &ret as *const Phase as *mut c_void);
                H5Gclose(raw_datas_group);
	    }
        } else {
	    unsafe { H5Fclose(file_id); }
	    return Err(format!("load_phase: failed opening raw datas group in file {}", filename));
        }

        // read peak_train channels
        let peaks_trains_group = unsafe { H5Gopen2(file_id,
                                                   CStr::from_bytes_with_nul("peaks_train\0".as_bytes())
                                                   .unwrap().as_ptr(),
                                                   H5P_DEFAULT) };
        if peaks_trains_group > 0 {
	    unsafe {
                H5Literate2(peaks_trains_group, 
			    H5_index_t_H5_INDEX_NAME,
			    H5_iter_order_t_H5_ITER_INC,
			    null_mut(),
			    Some(_load_peaks_trains),
			    &ret as *const Phase as *mut c_void);
	    }
        } else {
	    unsafe { H5Fclose(file_id); }
	    return Err(format!("load_phase: failed opening peaks trains group in file {}", filename));
        }

        unsafe { 
	    H5Gclose(peaks_trains_group);
	    H5Fclose(file_id);
        }
        Ok(ret)
    } else {
        Err(format!("load_phase: failed opening file {}", filename))
    }
}
