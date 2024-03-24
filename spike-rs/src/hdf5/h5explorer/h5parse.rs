use crate::hdf5::sys::*;
use crate::hdf5::utils::*;
use crate::hdf5::h5explorer::{H5Analog, H5Recording, H5Content};

pub extern "C" fn _inspect_analogs(group: i64,
                                   name:  *const i8,
                                   _info: *const H5L_info2_t,
    data:  *mut c_void) -> i32 {
    let recording = unsafe { &mut *(data as *mut H5Recording) };
    let inner_group = unsafe { H5Gopen2(group, name, H5P_DEFAULT) };

    // retrieve the stream path
    let mut path = cchar_to_string(name);
    path.insert_str(0, "/");
    path.insert_str(0, &recording.path);

    // retrieve the stream Label
    let mut analog = H5Analog::new(path);
    analog.label;
    analog.label = get_attr_str(inner_group, "Label\0", 64).expect(
        "Failed to retrieve analog stream Label");

    // retrieve the dimension of the stream
    unsafe {
        let channel_data_id = H5Dopen2(inner_group, str_to_cchar("ChannelData\0"), H5P_DEFAULT);
        let channel_data_space = H5Dget_space(channel_data_id);
        let channel_data_ndims = H5Sget_simple_extent_ndims(channel_data_space) as usize;
        analog.shape.resize(channel_data_ndims, 0);
        H5Sget_simple_extent_dims(channel_data_space, analog.shape.as_ptr() as _, null_mut());

        H5Sclose(channel_data_space);
        H5Dclose(channel_data_id);
    }


    recording.analogs.push(analog);
    unsafe { H5Gclose(inner_group) };
    0
}

pub extern "C" fn _inspect_events(group: i64,
                                   name:  *const i8,
                                   _info: *const H5L_info2_t,
    data:  *mut c_void) -> i32 {
    let recording = unsafe { &mut *(data as *mut H5Recording) };
    let inner_group = unsafe { H5Gopen2(group, name, H5P_DEFAULT) };

    0
}

pub extern "C" fn _inspect_recordings(group: i64,
                                      name:  *const i8,
                                      _info: *const H5L_info2_t,
                                      data:  *mut c_void) -> i32 {
    let mut content = unsafe { &mut *(data as *mut H5Content) };
    let inner_group = unsafe { H5Gopen2(group, name, H5P_DEFAULT) };
    let mut path = cchar_to_string(name);
    path.insert_str(0, "/Data/");
    let mut recording = H5Recording::new(path);
    
    // retrieve the duration of the recording
    recording.duration = get_attr_ilong(inner_group, "Duration\0").expect(
        "Failed to retrieve recording duration");

    // look for analog streams
    let analogs_id = unsafe { H5Gopen2(inner_group, str_to_cchar("AnalogStream\0"), H5P_DEFAULT) };
    if analogs_id > 0 {
        unsafe { H5Literate2(analogs_id,
                    H5_index_t_H5_INDEX_NAME,
                    H5_iter_order_t_H5_ITER_INC,
                    null_mut(),
                    Some(_inspect_analogs),
                    &recording as *const H5Recording as *mut c_void
        ) };
    }

    let events_id = unsafe { H5Gopen2(inner_group, str_to_cchar("EventStream\0"), H5P_DEFAULT) };
    if events_id > 0 {
        unsafe { H5Literate2(events_id,
                    H5_index_t_H5_INDEX_NAME,
                    H5_iter_order_t_H5_ITER_INC,
                    null_mut(),
                    Some(_inspect_events),
                    &recording as *const H5Recording as *mut c_void
        ) };
    } else {
        println!("{events_id}");
    }

    content.recordings.push(recording);
    unsafe { H5Gclose(inner_group) };
    0
}
