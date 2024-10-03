#include "pycode_h5.h"
#include <string.h>
#include <stddef.h>
#include <stdlib.h>

hid_t InfoChannelMemoryType;
hid_t InfoChannelStringType;

#define CAST(X, Y) (Y)(X)

//==============================================================================
//                      LIBRARY INITIALIZATION AND CLOSING
//==============================================================================
void pycodeh5_init() {
  H5open();
  InfoChannelMemoryType = H5Tcreate(H5T_COMPOUND, sizeof(InfoChannel));
  InfoChannelStringType = H5Tcopy(H5T_C_S1);
  H5Tset_size(InfoChannelStringType, SIZE_MAX);
  H5Tset_strpad(InfoChannelStringType, H5T_STR_NULLPAD);
  H5Tset_cset(InfoChannelStringType, H5T_CSET_ASCII);
  H5Tinsert(InfoChannelMemoryType, "ChannelID\0", offsetof(InfoChannel, channel_id), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "RowIndex\0", offsetof(InfoChannel, row_index), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "GroupId\0", offsetof(InfoChannel, group_id), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "ElectrodeGroup\0", offsetof(InfoChannel, electrode_group), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "Label\0", offsetof(InfoChannel, label), InfoChannelStringType); 
  H5Tinsert(InfoChannelMemoryType, "RawDataType\0", offsetof(InfoChannel, raw_data_type), InfoChannelStringType); 
  H5Tinsert(InfoChannelMemoryType, "Unit\0", offsetof(InfoChannel, unit), InfoChannelStringType); 
  H5Tinsert(InfoChannelMemoryType, "Exponent\0", offsetof(InfoChannel, exponent), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "AdZero\0", offsetof(InfoChannel, ad_zero), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "Tick\0", offsetof(InfoChannel, tick), H5T_NATIVE_LLONG);
  H5Tinsert(InfoChannelMemoryType, "ConversionFactor\0", offsetof(InfoChannel, conversion_factor), H5T_NATIVE_LLONG);
  H5Tinsert(InfoChannelMemoryType, "ADCBits\0", offsetof(InfoChannel, adc_bits), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "HighPassFilterType\0", offsetof(InfoChannel, high_pass_filter_type), InfoChannelStringType); 
  H5Tinsert(InfoChannelMemoryType, "HighPassFilterCutOff\0", offsetof(InfoChannel, high_pass_filter_cutoff), InfoChannelStringType); 
  H5Tinsert(InfoChannelMemoryType, "HighPassFilterOrder\0", offsetof(InfoChannel, high_pass_filter_order), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "LowPassFilterType\0", offsetof(InfoChannel, low_pass_filter_type), InfoChannelStringType); 
  H5Tinsert(InfoChannelMemoryType, "LowPassFilterCutOff\0", offsetof(InfoChannel, low_pass_filter_cutoff), InfoChannelStringType); 
  H5Tinsert(InfoChannelMemoryType, "LowPassFilterOrder\0", offsetof(InfoChannel, low_pass_filter_order), H5T_NATIVE_INT);
}

void pycodeh5_close() {
  H5Tclose(InfoChannelMemoryType);
  H5Tclose(InfoChannelStringType);
}

//==============================================================================
//                      ANALOG STREAMS RELATED FUNCTIONS 
//==============================================================================
typedef struct CallbackAnalogsRets {
  int current_index;
  AnalogStream *analog_streams;
} CallbackAnalogsRets;

/// count the number of analog groups in the AnalogStreams group
herr_t count_analogs_callback(hid_t group,
                              const char *name,
                              const H5L_info2_t *info,
                              void *op_data) {
  *((int *)op_data) += 1;
  return 0;
}
//==============================================================================

phaseh5_error open_analog(AnalogStream *analog_stream,
                           hid_t analog_stream_group) {

  // GET THE ANALOG STREAM LABEL
  // ----------------------------------------------------------------------
  hid_t label_attribute = H5Aopen(analog_stream_group, "Label", H5P_DEFAULT);
  if (label_attribute <= 0) {
    return OPEN_LABEL_ATTRIBUTE_FAIL;
  }

  hid_t label_datatype = H5Aget_type(label_attribute);
  if (label_datatype <= 0) {
    return OPEN_LABEL_DATATYPE_FAIL;
  }

  herr_t res = H5Aread(label_attribute, label_datatype, (void *)analog_stream);
  if (res < 0) {
    return READ_LABEL_ATTRIBUTE_FAIL;
  }

  H5Tclose(label_datatype);
  H5Aclose(label_attribute);

  // ----------------------------------------------------------------------
  // PARSE THE InfoChannel dataset
  // ----------------------------------------------------------------------

  hid_t info_channel_dataset = H5Dopen2(analog_stream_group, "InfoChannel", H5P_DEFAULT);
  if (info_channel_dataset <= 0) {
    return OPEN_INFO_CHANNEL_DATASET_FAIL;
  }

  hid_t info_channel_dataspace = H5Dget_space(info_channel_dataset);
  if (info_channel_dataspace <= 0) {
    return OPEN_INFO_CHANNEL_DATASPACE_FAIL;
  }

  hid_t info_channel_datatype = H5Dget_type(info_channel_dataset);
  if (info_channel_datatype <= 0) {
    return OPEN_INFO_CHANNEL_DATATYPE_FAIL;
  }

  H5Sget_simple_extent_dims(info_channel_dataspace, &analog_stream->n_channels, NULL);
  hsize_t memory_rank[] = {analog_stream->n_channels};
  hid_t memspace_id = H5Screate_simple(1, memory_rank, NULL);

  res = H5Dread(info_channel_dataset, InfoChannelMemoryType, memspace_id,
                H5S_ALL, H5P_DEFAULT, (void *)analog_stream->info_channels);

  if (res < 0) {
    return READ_INFO_CHANNELS_FAIL;
  }

  H5Tclose(info_channel_datatype);
  H5Sclose(info_channel_dataspace);
  H5Dclose(info_channel_dataset);

  // ----------------------------------------------------------------------
  // Get the handle for the ChannelData stream
  // ----------------------------------------------------------------------
  hid_t channel_data_dataset = H5Dopen2(analog_stream_group, "ChannelData", H5P_DEFAULT);
  if (channel_data_dataset <= 0) {
    return OPEN_CHANNEL_DATA_FAIL;
  }

  hid_t channel_data_dataspace = H5Dget_space(channel_data_dataset);
  if (channel_data_dataspace <= 0) {
    return OPEN_CHANNEL_DATA_DATASPACE_FAIL;
  }

  hsize_t dims[2];
  res = H5Sget_simple_extent_dims(channel_data_dataspace, dims, NULL);
  if (res < 0) {
    return GET_CHANNEL_DATA_DIMS_FAIL;
  }

  analog_stream->datalen = dims[1];

  analog_stream->channel_data_dataset = channel_data_dataset;
  return OK;
}

/// parse an analog
herr_t open_analogs_callback(hid_t group,
                             const char *name,
                             const H5L_info2_t *info,
                             void *analogs_rets) {
  hid_t analog_stream_group = H5Gopen2(group, name, H5P_DEFAULT);
  if (analog_stream_group <= 0) {
    return OPEN_ANALOG_GROUP_FAIL;
  }

  CallbackAnalogsRets* analog_rets_c = CAST(analogs_rets, CallbackAnalogsRets*);
  int res = open_analog(&analog_rets_c->analog_streams[analog_rets_c->current_index],
                        analog_stream_group);
  if (res != OK) {
    printf("open_analogs_callback ERROR %d", res);
    return res;
  }
  analog_rets_c->current_index += 1;

  H5Gclose(analog_stream_group);
  return 0;
}

phaseh5_error close_analog(AnalogStream* analog_stream) {
  
H5Dclose(analog_stream->channel_data_dataset);
  return OK;
}

//==============================================================================
//                      PHASE RELATED FUNCTIONS 
//==============================================================================
void init_phase(PhaseH5 *phase) { memset(phase, 0, sizeof(PhaseH5)); }

phaseh5_error phase_open(PhaseH5 *phase, const char *filename) {
  hid_t fid = H5Fopen(filename, H5P_DEFAULT, H5P_DEFAULT);
  if (fid <= 0) {
    return OPEN_FAIL;
  }

  hid_t data_group = H5Gopen2(fid, "/Data", H5P_DEFAULT);
  if (data_group <= 0) {
    return OPEN_DATA_GROUP_FAIL;
  }

  // GET THE DATE OF THE RECORDING
  // ----------------------------------------------------------------------
  hid_t date_attribute = H5Aopen(data_group, "Date", H5P_DEFAULT);
  H5Gclose(data_group);
  if (date_attribute <= 0) {
    return OPEN_DATE_ATTRIBUTE_FAIL;
  }

  hid_t date_datatype = H5Aget_type(date_attribute);
  if (date_datatype <= 0) {
    return OPEN_DATE_DATATYPE_FAIL;
  }

  herr_t res = H5Aread(date_attribute, date_datatype, (void *)phase->date);
  if (res < 0) {
    return READ_DATE_ATTRIBUTE_FAIL;
  }

  H5Tclose(date_datatype);
  H5Aclose(date_attribute);

  // ----------------------------------------------------------------------
  // PARSE THE ANALOG STREAMS
  // ----------------------------------------------------------------------
  hid_t analog_group =
      H5Gopen2(fid, "/Data/Recording_0/AnalogStream", H5P_DEFAULT);
  if (analog_group <= 0) {
    return OPEN_ANALOG_GROUP_FAIL;
  }

  int n_analogs = 0;
  res = H5Literate2(analog_group, H5_INDEX_NAME, H5_ITER_NATIVE, NULL,
                    count_analogs_callback, (void *)(&n_analogs));
  if (res != 0) {
    return res;
  }

  AnalogStream analog_streams[n_analogs];
  CallbackAnalogsRets callback_ret = {
      .current_index = 0,
      .analog_streams = analog_streams,
  };

  res = H5Literate2(analog_group, H5_INDEX_NAME, H5_ITER_NATIVE, NULL,
                    open_analogs_callback, (void *)&callback_ret);
  if (res != 0) {
    return res;
  }

  long raw_data_index = -1;
  long digital_index = -1;
  float sampling_frequency = -1;
  long datalen = -1;

  for (int i = 0; i<n_analogs; ++i) {
    // test that there is only a raw data stream and only a digital stream
    // digital stream case
    if (analog_streams[i].n_channels == 1) {
      if (digital_index == -1) digital_index = i;
      else {
        return MULTIPLE_DIGITAL_STREAMS;
      }
      // test that all the channels have the same sampling frequency
      if (sampling_frequency == -1) {
        sampling_frequency = analog_streams[i].info_channels[0].tick * 100;
      } else if (sampling_frequency != analog_streams[i].info_channels[0].tick * 100){
        return MULTIPLE_SAMPLING_FREQUENCIES;
      }
    } else {
      // raw data stream case
      if (raw_data_index == -1) raw_data_index = i;
      else {
        return MULTIPLE_RAW_DATA_STREAMS; 
      }
      // test that all the channels have the same sampling frequency
      if (sampling_frequency == -1) {
        sampling_frequency = analog_streams[i].info_channels[0].tick * 100;
      } else {
        for (int j = 0; j<analog_streams[i].n_channels; ++j) {
          if (sampling_frequency != analog_streams[i].info_channels[j].tick * 100){
            return MULTIPLE_SAMPLING_FREQUENCIES;
          }
        }
      }
    }
    // test that all the channels have the same datalen
    if (datalen == -1) {
      datalen = analog_streams[i].datalen;
    } else {
      if (datalen != analog_streams[i].datalen) {
        return MULTIPLE_DATALENS;
      }
    }
  }

  phase->datalen = datalen;
  phase->sampling_frequency = sampling_frequency;

  if (raw_data_index == -1) {
    return NO_RAW_DATA_STREAM;
  } else {
    memcpy(&phase->raw_data, &analog_streams[raw_data_index], sizeof(AnalogStream));
  }
  if (digital_index == -1) {
    phase->has_digital = false;
  } else {
    memcpy(&phase->digital, &analog_streams[digital_index], sizeof(AnalogStream));
    phase->has_digital = true;
  }

  H5Gclose(analog_group);
  // ----------------------------------------------------------------------

  H5Fclose(fid);
  return OK;
}

phaseh5_error phase_close(PhaseH5* phase) {
  close_analog(&phase->raw_data);
  if (phase->has_digital) {
    close_analog(&phase->digital);
  }
  return OK;
}

//==============================================================================
//                      RAW DATA I/O FUNCTIONS 
//==============================================================================
phaseh5_error raw_data(PhaseH5* phase, size_t index, size_t start, size_t end, float* buf) {
  if (end < start) {
    return RAW_DATA_END_BEFORE_START;
  }

  if (end >= phase->datalen) {
    return RAW_DATA_END_OUT_OF_BOUNDS;
  }

  size_t dims[] = {end - start};
  hid_t raw_data_dataspace = H5Dget_space(phase->raw_data.channel_data_dataset);

  if (raw_data_dataspace <= 0) {
    return RAW_DATA_GET_DATASPACE_FAIL;
  }
  
  size_t _start[] = {index, start};
  size_t _count[] = {index, end - start};
  herr_t res = H5Sselect_hyperslab(raw_data_dataspace, H5S_SELECT_SET, _start, NULL, _count, NULL);

  if (res < 0) {
    return RAW_DATA_SELECT_HYPERSLAB_FAIL;
  }

  hid_t read_dataspace = H5Screate_simple(1, dims, NULL);
  if (read_dataspace <= 0) {
    return RAW_DATA_CREATE_MEMORY_DATASPACE_FAIL;
  }

  int buffer[end-start];

  res = H5Dread(phase->raw_data.channel_data_dataset, H5T_NATIVE_INT, read_dataspace,
          raw_data_dataspace, H5P_DEFAULT, buffer);

  if (res < 0) {
    return RAW_DATA_READ_DATA_FAIL;
  }

  return OK;
}

phaseh5_error set_raw_data(PhaseH5* phase, size_t index, size_t start, size_t end, float *buf) {
  if (end < start) {
    return RAW_DATA_END_BEFORE_START;
  }

  // if (end >= phase->datalen) {
  //   return RAW_DATA_END_OUT_OF_BOUNDS;
  // }

  return OK;
}
