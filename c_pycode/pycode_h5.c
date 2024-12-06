//TODO! adapt pycodeh5_init with RES macro

#include "pycode_h5.h"
#include <string.h>
#include <stddef.h>
#include <stdlib.h>

hid_t InfoChannelMemoryType;
hid_t HDF5StringType;

#define CAST(X, Y) (Y)(X)
// #define RES(F, E) {                           \
//   herr_t res = F;                               \
//   if (res < 0) {                                \ 
//     return E;                                   \
//   }                                             \               
// }


//==============================================================================
//                      LIBRARY INITIALIZATION AND CLOSING
//==============================================================================
phaseh5_error pycodeh5_init() {
  H5open();
  InfoChannelMemoryType = H5Tcreate(H5T_COMPOUND, sizeof(InfoChannel));
  HDF5StringType = H5Tcopy(H5T_C_S1);
  H5Tset_size(HDF5StringType, SIZE_MAX);
  H5Tset_strpad(HDF5StringType, H5T_STR_NULLPAD);
  H5Tset_cset(HDF5StringType, H5T_CSET_ASCII);
  H5Tinsert(InfoChannelMemoryType, "ChannelID\0", offsetof(InfoChannel, channel_id), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "RowIndex\0", offsetof(InfoChannel, row_index), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "GroupId\0", offsetof(InfoChannel, group_id), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "ElectrodeGroup\0", offsetof(InfoChannel, electrode_group), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "Label\0", offsetof(InfoChannel, label), HDF5StringType); 
  H5Tinsert(InfoChannelMemoryType, "RawDataType\0", offsetof(InfoChannel, raw_data_type), HDF5StringType); 
  H5Tinsert(InfoChannelMemoryType, "Unit\0", offsetof(InfoChannel, unit), HDF5StringType); 
  H5Tinsert(InfoChannelMemoryType, "Exponent\0", offsetof(InfoChannel, exponent), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "AdZero\0", offsetof(InfoChannel, ad_zero), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "Tick\0", offsetof(InfoChannel, tick), H5T_NATIVE_LLONG);
  H5Tinsert(InfoChannelMemoryType, "ConversionFactor\0", offsetof(InfoChannel, conversion_factor), H5T_NATIVE_LLONG);
  H5Tinsert(InfoChannelMemoryType, "ADCBits\0", offsetof(InfoChannel, adc_bits), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "HighPassFilterType\0", offsetof(InfoChannel, high_pass_filter_type), HDF5StringType); 
  H5Tinsert(InfoChannelMemoryType, "HighPassFilterCutOff\0", offsetof(InfoChannel, high_pass_filter_cutoff), HDF5StringType); 
  H5Tinsert(InfoChannelMemoryType, "HighPassFilterOrder\0", offsetof(InfoChannel, high_pass_filter_order), H5T_NATIVE_INT);
  H5Tinsert(InfoChannelMemoryType, "LowPassFilterType\0", offsetof(InfoChannel, low_pass_filter_type), HDF5StringType); 
  H5Tinsert(InfoChannelMemoryType, "LowPassFilterCutOff\0", offsetof(InfoChannel, low_pass_filter_cutoff), HDF5StringType); 
  H5Tinsert(InfoChannelMemoryType, "LowPassFilterOrder\0", offsetof(InfoChannel, low_pass_filter_order), H5T_NATIVE_INT);

  return OK;
}

void pycodeh5_close() {
  H5Tclose(InfoChannelMemoryType);
  H5Tclose(HDF5StringType);
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
//                      EVENTS RELATED FUNCTIONS 
//==============================================================================

/// count the number of analog groups in the AnalogStreams group
herr_t count_events_callback(hid_t group,
                             const char *name,
                             const H5L_info2_t *info,
                             void *op_data) {
  if (!strncmp(name, "EventEntity_", sizeof("EventEntity_")/sizeof(char)-1)) {
    *((int *)op_data) += 1;
  }
  return 0;
}

typedef struct CallbackEventsRets {
  int current_index;
  hid_t* event_entities;
} CallbackEventsRets;


herr_t open_events_callback(hid_t group,
                            const char *name,
                            const H5L_info2_t *info,
                            void *events_rets) {
  if (!strncmp(name, "EventEntity_", sizeof("EventEntity_")/sizeof(char)-1)) {
    CallbackEventsRets* events_rets_c = CAST(events_rets, CallbackEventsRets*);
    if (events_rets_c->current_index == MAX_EVENT_STREAMS - 1) {
      return MAX_EVENT_STREAMS_EXCEEDED;
    }
    hid_t entity_dataset = H5Dopen2(group, name, H5P_DEFAULT);
    if (entity_dataset <= 0) {
    return OPEN_ENTITY_DATASET_FAIL;
    }
    events_rets_c->event_entities[events_rets_c->current_index] = entity_dataset;
    events_rets_c->current_index += 1;
  }
  return OK;
}

//==============================================================================
//                      PHASE RELATED FUNCTIONS 
//==============================================================================
void init_phase(PhaseH5 *phase) { memset(phase, 0, sizeof(PhaseH5)); }

phaseh5_error phase_open(PhaseH5 *phase, const char *filename) {
  hid_t fid = H5Fopen(filename, H5F_ACC_RDWR, H5P_DEFAULT);
  if (fid <= 0) {
    return OPEN_FAIL;
  }
  phase->fid = fid;

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

  AnalogStream *analog_streams = (AnalogStream*)calloc(n_analogs, sizeof(AnalogStream));
  // AnalogStream analog_streams[MAX_ANALOG_STREAMS] = {0};
  if (analog_streams == NULL) {
    return OPEN_ALLOCATE_ANALOGS_FAIL;
  }

  CallbackAnalogsRets callback_ret = {
      .current_index = 0,
      .analog_streams = analog_streams,
  };

  res = H5Literate2(analog_group, H5_INDEX_NAME, H5_ITER_NATIVE, NULL,
                    open_analogs_callback, (void *)&callback_ret);
  if (res != 0) {
    return res;
  }

  size_t raw_data_index;
  size_t digital_index;
  bool raw_data_set = false;
  bool digital_set = false;

  float sampling_frequency = -1;
  long datalen = -1;

  for (long int i = 0; i<n_analogs; ++i) {
    // test that there is only a raw data stream and only a digital stream
    // digital stream case
    if (analog_streams[i].n_channels == 1) {
      if (digital_set == false) {
        digital_index = i;
        digital_set = true;
      }
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
      if (raw_data_set == false) {
        raw_data_index = i;
        raw_data_set = true;
      }
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

  // printf("raw DATA INDEX: %ld\nDIGITAL_INDEX: %ld\n", raw_data_index, digital_index);

  if (raw_data_set == false) {
    return NO_RAW_DATA_STREAM;
  } else {
    memcpy(&phase->raw_data, analog_streams + raw_data_index, sizeof(AnalogStream));
  }
  if (digital_set == false) {
    phase->has_digital = false;
  } else {
    memcpy(&phase->digital, analog_streams + digital_index, sizeof(AnalogStream));
    phase->has_digital = true;
  }

  H5Gclose(analog_group);
  free(analog_streams);
  
  // ----------------------------------------------------------------------
  // PARSE THE EVENT STREAMS
  // ----------------------------------------------------------------------

  res = H5Lexists(fid, "/Data/Recording_0/EventStream", H5P_DEFAULT);

  if (res < 0) {
    return OPEN_EVENT_STREAM_GROUP_LINK_FAIL;
  } else if (res == 0) {
    phase->n_events = 0;
  } else {
    hid_t event_stream = H5Gopen2(fid, "/Data/Recording_0/EventStream", H5P_DEFAULT);
    if (event_stream <= 0) {
      return OPEN_EVENT_STREAM_GROUP_FAIL;
    }
    res = H5Lexists(fid, "/Data/Recording_0/EventStream/Stream_0", H5P_DEFAULT);
    if (res < 0) {
      return OPEN_EVENT_STREAM_STREAM_0_GROUP_LINK_FAIL;
    } else if (res == 0) {
      phase->n_events = 0;
    } else {
      hid_t events_group = H5Gopen2(fid, "/Data/Recording_0/EventStream/Stream_0", H5P_DEFAULT);
      if (events_group <= 0) {
        return OPEN_EVENT_STREAM_GROUP_FAIL;
      }

      res = H5Literate2(events_group,
                        H5_INDEX_NAME,
                        H5_ITER_NATIVE,
                        NULL,
                        count_events_callback,
                        &phase->n_events);
      if (res != OK) {
        return res;
      }

      CallbackEventsRets events_rets;
      events_rets.current_index = 0;
      res = H5Literate2(events_group,
                        H5_INDEX_NAME,
                        H5_ITER_NATIVE,
                        NULL,
                        open_events_callback,
                        &events_rets);

      if (res != OK) {
        return res;
      }

      for (int i=0; i<events_rets.current_index; ++i) {
        phase->event_entities[i] = events_rets.event_entities[i];
      }
    }
  }
  
  // ----------------------------------------------------------------------
  // PARSE THE PEAK_TRAIN GROUP
  // ----------------------------------------------------------------------

  res = H5Lexists(fid, "/Data/Recording_0/Peak_Train", H5P_DEFAULT);

  if (res < 0) {
    phase->peaks_group = false;
    return OPEN_PEAK_TRAIN_GROUP_FAIL;
  } else if (res == 0) {
    res = H5Gcreate(fid, "/Data/Recording_0/Peak_Train", H5P_DEFAULT, H5P_DEFAULT, H5P_DEFAULT);
    if (res < 0) {
      return CREATE_PEAK_GROUP_FAIL;
    }
    phase->peaks_group = true;
  } else {
    phase->peaks_group = true;
  }

  return OK;
}

phaseh5_error phase_close(PhaseH5* phase) {
  herr_t res;
  close_analog(&phase->raw_data);
  if (phase->has_digital) {
    close_analog(&phase->digital);
  }
  for (int i=0; i<phase->n_events; i++) {
    res = H5Dclose(phase->event_entities[i]);
    if (res < 0) {
      return EVENT_ENTITY_DATASET_CLOSE_FAIL;
    }
  }
  res = H5Fclose(phase->fid);
  if (res < 0) {
    return CLOSE_FILE_FAIL;
  }
  return OK;
}

//==============================================================================
//                      RAW DATA I/O FUNCTIONS 
//==============================================================================
phaseh5_error raw_data(PhaseH5* phase, size_t index, size_t start, size_t end, int* buf) {
  if (end < start) {
    return RAW_DATA_END_BEFORE_START;
  }

  if (end >= phase->datalen) {
    return RAW_DATA_END_OUT_OF_BOUNDS;
  }

  size_t dims[] = {end - start};
  hid_t raw_data_dataspace = H5Dget_space(phase->raw_data.channel_data_dataset);
  // TODO! fix one sample left over maybe here, maybe in the rust code but if
  // you query for all the samples it gives you back all but one.

  if (raw_data_dataspace <= 0) {
    return RAW_DATA_GET_DATASPACE_FAIL;
  }
  
  size_t _start[] = {index, start};
  size_t _count[] = {1, end - start};
  herr_t res = H5Sselect_hyperslab(raw_data_dataspace, H5S_SELECT_SET, _start, NULL, _count, NULL);

  if (res < 0) {
    return RAW_DATA_SELECT_HYPERSLAB_FAIL;
  }

  hid_t read_dataspace = H5Screate_simple(1, dims, NULL);
  if (read_dataspace <= 0) {
    return RAW_DATA_CREATE_MEMORY_DATASPACE_FAIL;
  }

  res = H5Dread(phase->raw_data.channel_data_dataset, H5T_NATIVE_INT, read_dataspace,
          raw_data_dataspace, H5P_DEFAULT, buf);
  
  if (res < 0) {
    return RAW_DATA_READ_DATA_FAIL;
  }

  return OK;
}

phaseh5_error set_raw_data(PhaseH5* phase, size_t index, size_t start, size_t end, const int *buf) {
  if (end < start) {
    return SET_RAW_DATA_END_BEFORE_START;
  }

  if (end >= phase->datalen) {
    return SET_RAW_DATA_END_OUT_OF_BOUNDS;
  }

  // get the ChannelData dataspace
  hid_t channel_data_dataspace = H5Dget_space(phase->raw_data.channel_data_dataset);
  if (channel_data_dataspace <= 0) {
    return SET_RAW_DATA_GET_DATASPACE_FAIL;
  }

  hsize_t s_start[] = {index, start};
  hsize_t s_count[] = {1, end-start};

  printf("index: %ld\nstart: %ld\nend: %ld\n", index, start, end);

  // set the subspace of the dataspace where to write
  herr_t res = H5Sselect_hyperslab(channel_data_dataspace, H5S_SELECT_SET,
                                   s_start, NULL, s_count, NULL);

  if (res < 0) {
    return SET_RAW_DATA_SELECT_HYPERSLAB_FAIL;
  }

  hsize_t memory_dataspace_dims[] = {end-start};
  hid_t memory_dataspace = H5Screate_simple(1, memory_dataspace_dims, NULL);
  if (memory_dataspace <= 0) {
    return SET_RAW_DATA_CREATE_MEMORY_DATASPACE_FAIL;
  }

  res = H5Dwrite(phase->raw_data.channel_data_dataset,
                 H5T_NATIVE_INT,
                 memory_dataspace,
                 channel_data_dataspace,
                 H5P_DEFAULT,
                 buf);

  if (res < 0) {
    return SET_RAW_DATA_WRITE_DATASET_FAIL;
  }
  
  return OK;
}

phaseh5_error digital(PhaseH5* phase, size_t start, size_t end, int* buf) {
  if (!phase->has_digital) {
    return DIGITAL_NO_DIGITAL;
  }
  
  if (end < start) {
    return DIGITAL_END_BEFORE_START;
  }

  if (end >= phase->datalen) {
    return DIGITAL_END_OUT_OF_BOUNDS;
  }

  size_t dims[] = {end - start};
  hid_t digital_dataspace = H5Dget_space(phase->digital.channel_data_dataset);

  if (digital_dataspace <= 0) {
    return DIGITAL_GET_DATASPACE_FAIL;
  }
  
  size_t _start[] = {0, start};
  size_t _count[] = {1, end - start};
  herr_t res = H5Sselect_hyperslab(digital_dataspace, H5S_SELECT_SET, _start, NULL, _count, NULL);

  if (res < 0) {
    return DIGITAL_SELECT_HYPERSLAB_FAIL;
  }

  hid_t read_dataspace = H5Screate_simple(1, dims, NULL);
  if (read_dataspace <= 0) {
    return DIGITAL_CREATE_MEMORY_DATASPACE_FAIL;
  }

  res = H5Dread(phase->digital.channel_data_dataset, H5T_NATIVE_INT, read_dataspace,
          digital_dataspace, H5P_DEFAULT, buf);
  
  if (res < 0) {
    return DIGITAL_READ_DATA_FAIL;
  }

  return OK;
}

phaseh5_error set_digital(PhaseH5* phase, size_t start, size_t end, const int *buf) {
  if (!phase->has_digital) {
    return SET_DIGITAL_NO_DIGITAL;
  }
  
  if (end < start) {
    return SET_DIGITAL_END_BEFORE_START;
  }

  if (end >= phase->datalen) {
    return SET_DIGITAL_END_OUT_OF_BOUNDS;
  }

  // get the ChannelData dataspace
  hid_t channel_data_dataspace = H5Dget_space(phase->raw_data.channel_data_dataset);
  if (channel_data_dataspace <= 0) {
    return SET_DIGITAL_GET_DATASPACE_FAIL;
  }

  hsize_t s_start[] = {0, start};
  hsize_t s_count[] = {1, end-start};

  // set the subspace of the dataspace where to write
  herr_t res = H5Sselect_hyperslab(channel_data_dataspace, H5S_SELECT_SET,
                                   s_start, NULL, s_count, NULL);

  if (res < 0) {
    return SET_DIGITAL_SELECT_HYPERSLAB_FAIL;
  }

  hsize_t memory_dataspace_dims[] = {end-start};
  hid_t memory_dataspace = H5Screate_simple(1, memory_dataspace_dims, NULL);
  if (memory_dataspace <= 0) {
    return SET_DIGITAL_CREATE_MEMORY_DATASPACE_FAIL;
  }

  res = H5Dwrite(phase->raw_data.channel_data_dataset,
                 H5T_NATIVE_INT,
                 memory_dataspace,
                 channel_data_dataspace,
                 H5P_DEFAULT,
                 buf);

  if (res < 0) {
    return SET_DIGITAL_WRITE_DATA_FAIL;
  }
  
  return OK;
}

phaseh5_error events_len(PhaseH5* phase, size_t index, hsize_t *dim) {
  if (index >= phase->n_events) {
    return EVENTS_LEN_INDEX_OUT_OF_BOUNDS;
  }
  hid_t event_dataset = phase->event_entities[index];
  hid_t event_dataspace = H5Dget_space(event_dataset);
  if (event_dataspace <= 0) {
    return EVENTS_LEN_OPEN_EVENT_DATASPACE_FAIL;
  }
  hsize_t dims[2];
  herr_t res = H5Sget_simple_extent_dims(event_dataspace, dims, NULL);
  if (res < 0 ) {
    return EVENTS_LEN_GET_DIMS_FAIL;
  }
  *dim = dims[1];

  return OK;
}

phaseh5_error events(PhaseH5* phase, size_t index, LLONG_TYPE *buf) {
  if (index >= phase->n_events) {
    return EVENTS_INDEX_OUT_OF_BOUNDS;
  }
  hsize_t dim;
  herr_t res = events_len(phase, index, &dim);
  if (res != OK) {
    return res;
  }

  hid_t events_dataset = phase->event_entities[index];
  hid_t file_dataspace = H5Dget_space(events_dataset);
  if (file_dataspace <= 0) {
    return EVENTS_GET_EVENTS_DATASPACE_FAIL;
  }
  hsize_t start[] = {0, 0};
  hsize_t count[] = {1, dim};
  res = H5Sselect_hyperslab(file_dataspace,
                            H5S_SELECT_SET,
                            start,
                            NULL,
                            count,
                            NULL);
  if (res < 0) {
    return EVENTS_SELECT_DATASPACE_HYPERSLAB_FAIL;
  }

  hsize_t memory_dim[] = {dim};

  hid_t memory_dataspace = H5Screate_simple(1, memory_dim, NULL);
  if (memory_dataspace <= 0) {
    return EVENTS_CREATE_MEMORY_DATASPACE_FAIL;
  }

  res = H5Dread(events_dataset,
                H5T_NATIVE_LONG,
                memory_dataspace,
                file_dataspace,
                H5P_DEFAULT,
                buf);

  if (res < 0) {
    return EVENTS_READ_DATASET_FAIL;
  }
  
  return OK;
}

phaseh5_error open_peak_train_datasets(PhaseH5* phase, const char* label, hid_t* values, hid_t* samples) {
  // Check if there are peak train data in the file
  if (phase->peaks_group == 0) {
    return PEAK_TRAIN_NO_PEAK_GROUP;
  }

  // Get the path of peak train datasets of that label
  hsize_t values_len;
  hsize_t samples_len;

  char peak_train_group_str[MAX_GROUP_STRING_LEN] = {0};
  char values_group_str[MAX_GROUP_STRING_LEN] = {0};
  char samples_group_str[MAX_GROUP_STRING_LEN] = {0};

  sprintf(peak_train_group_str, "/Data/Recording_0/Peak_Train/%s/", label);

  herr_t res = H5Lexists(phase->fid, peak_train_group_str, H5P_DEFAULT);
  if (res < 0) {
    return PEAK_TRAIN_GROUP_LINK_FAIL;
  } else if (res == 0) {
    return PEAK_TRAIN_NO_PEAK_GROUP;
  }
  
  sprintf(values_group_str, "/Data/Recording_0/Peak_Train/%s/values", label);
  sprintf(samples_group_str, "/Data/Recording_0/Peak_Train/%s/samples", label);

  // Check if those links exist
  res = H5Lexists(phase->fid, values_group_str, H5P_DEFAULT);
  if (res < 0) {
    return PEAK_TRAIN_VALUES_DATASET_LINK_FAIL;
  } else if (res == 0) {
    return PEAK_TRAIN_NO_VALUES_DATASET;
  } 

  res = H5Lexists(phase->fid, samples_group_str, H5P_DEFAULT);
  if (res < 0 ) {
    return PEAK_TRAIN_SAMPLES_DATASET_LINK_FAIL;
  } else if (res == 0) {
    return PEAK_TRAIN_NO_SAMPLES_DATASET;
  }

  // Open the datasets
  hid_t values_ds = H5Dopen2(phase->fid, values_group_str, H5P_DEFAULT);
  if (values_ds <= 0) {
    return PEAK_TRAIN_OPEN_VALUES_DATASET_FAIL;
  }
  *values = values_ds;

  hid_t samples_ds = H5Dopen2(phase->fid, samples_group_str, H5P_DEFAULT);
  if (samples_ds <= 0) {
    return PEAK_TRAIN_OPEN_SAMPLES_DATASET_FAIL;
  }
  *samples = samples_ds;

  return OK;
}

phaseh5_error peak_train_len(PhaseH5* phase, const char* label, size_t *len) {
  hid_t values_ds;
  hid_t samples_ds;
  herr_t res = open_peak_train_datasets(phase, label, &values_ds, &samples_ds);
  if (res != OK) {
    return res;
  }

  hsize_t values_len[1];
  hsize_t samples_len[1];
  
  hid_t values_dataspace = H5Dget_space(values_ds);
  if (values_dataspace <= 0) {
    return PEAK_TRAIN_LEN_OPEN_VALUES_DATASPACE_FAIL;
  }

  res = H5Sget_simple_extent_dims(values_dataspace, values_len, NULL);
  if (res < 0) {
    return PEAK_TRAIN_LEN_GET_VALUES_DATASPACE_DIM_FAIL;
  }

  res = H5Sclose(values_dataspace);
  if (res < 0) {
    return PEAK_TRAIN_LEN_CLOSE_VALUES_DATASPACE_FAIL;
  }

  hid_t samples_dataspace = H5Dget_space(samples_ds);
  if (samples_dataspace <= 0) {
    return PEAK_TRAIN_LEN_OPEN_SAMPLES_DATASPACE_FAIL;
  }

  res = H5Sget_simple_extent_dims(samples_dataspace, samples_len, NULL);
  if (res < 0) {
    return PEAK_TRAIN_LEN_GET_SAMPLES_DATASPACE_DIM_FAIL;
  }

  res = H5Sclose(samples_dataspace);
  if (res < 0) {
    return PEAK_TRAIN_LEN_CLOSE_SAMPLES_DATASPACE_FAIL;
  }

  if (values_len[0] != samples_len[0]) {
    return PEAK_TRAIN_LEN_VALUES_SAMPLES_DIFFERENT;
  }

  *len = values_len[0];

  res = H5Dclose(values_ds);
  if (res < 0) {
    return PEAK_TRAIN_LEN_CLOSE_VALUES_DATASET_FAIL;
  }

  res = H5Dclose(samples_ds);
  if (res < 0) {
    return PEAK_TRAIN_LEN_CLOSE_SAMPLES_DATASET_FAIL;
  }

  return OK;
}

phaseh5_error peak_train(PhaseH5* phase, const char* label, PeakTrain* peak_train) {
  // Open peak dataset
  hid_t values_ds;
  hid_t samples_ds;
  herr_t res = open_peak_train_datasets(phase, label, &values_ds, &samples_ds);
  if (res != OK) {
    return res;
  }

  // Create memory dataspace
  hsize_t n_spikes[1] = {peak_train->n_peaks};
  
  hid_t memory_dataspace = H5Screate_simple(1, n_spikes, NULL);
  if (memory_dataspace <= 0) {
    return PEAK_TRAIN_CREATE_MEMORY_DATASPACE_FAIL;
  }

  // Read the datasets
  res = H5Dread(values_ds, H5T_NATIVE_FLOAT, memory_dataspace, H5S_ALL, H5P_DEFAULT, peak_train->values);
  if (res < 0) {
    return PEAK_TRAIN_READ_VALUES_DATASET_FAIL;
  }
  res = H5Dread(samples_ds, H5T_NATIVE_ULONG, memory_dataspace, H5S_ALL, H5P_DEFAULT, peak_train->samples);
  if (res < 0) {
    return PEAK_TRAIN_READ_SAMPLES_DATASET_FAIL;
  }

  // Close the opened identifiers
  res = H5Sclose(memory_dataspace);
  if (res < 0) {
    return PEAK_TRAIN_CLOSE_MEMORY_DATASPACE_FAIL;
  }

  res = H5Dclose(values_ds);
  if (res < 0) {
    return PEAK_TRAIN_CLOSE_VALUES_DATASET_FAIL;
  }

  res = H5Dclose(samples_ds);
  if (res < 0) {
    return PEAK_TRAIN_CLOSE_SAMPLES_DATASET_FAIL;
  }

  return OK;
}

/*
TODO! restructure this function
it must:
- check if the group for the selected label already exists.
- if exists:
  - get and update the data
  - delete the old dataset
- create the new dataset
- write into them
 */
phaseh5_error set_peak_train(PhaseH5* phase, const char* label, const PeakTrain* peak_train) {
  char values_group_str[MAX_GROUP_STRING_LEN];
  char samples_group_str[MAX_GROUP_STRING_LEN];

  sprintf(values_group_str, "/Data/Recording_0/Peak_Train/%s/values", label);
  sprintf(samples_group_str, "/Data/Recording_0/Peak_Train/%s/samples", label);

  // Delete old dataspaces if present (maybe close the identifiers)
  // Check if the group exists
  char label_group_str[MAX_GROUP_STRING_LEN];
  sprintf(label_group_str, "/Data/Recording_0/Peak_Train/%s/", label);
  herr_t res = H5Lexists(phase->fid, label_group_str, H5P_DEFAULT);

  if (res < 0) {
    return SET_PEAK_TRAIN_CHECK_LABEL_GROUP_FAIL;
  } else if (res > 0) {
    // the group exists. Delete the old datasets

    //   Check if those links exist
    res = H5Lexists(phase->fid, values_group_str, H5P_DEFAULT);
    if (res < 0) {
      return DELETE_PEAK_TRAIN_VALUES_DATASET_LINK_FAIL;
    } else if (res == 0) {
      // return DELETE_PEAK_TRAIN_NO_VALUES_DATASET;
    } else {
      res = H5Ldelete(phase->fid, values_group_str, H5P_DEFAULT);
      if (res < 0) {
        return DELETE_PEAK_TRAIN_VALUES_DATASET_FAIL;
      }
    }

    res = H5Lexists(phase->fid, samples_group_str, H5P_DEFAULT);
    if (res < 0 ) {
      return DELETE_PEAK_TRAIN_SAMPLES_DATASET_LINK_FAIL;
    } else if (res == 0) {
      // return DELETE_PEAK_TRAIN_NO_SAMPLES_DATASET;
    } else {
      res = H5Ldelete(phase->fid, samples_group_str, H5P_DEFAULT);
      if (res < 0) {
        return DELETE_PEAK_TRAIN_SAMPLES_DATASET_FAIL;
      }
    }
  } else {
    // There is no group. Create it.
    res = H5Gcreate2(phase->fid, label_group_str, H5P_DEFAULT, H5P_DEFAULT, H5P_DEFAULT);
    if (res <= 0) {
      return SET_PEAK_TRAIN_CREATE_GROUP_FAIL;
    }
  }

  // Create memory dataspace for the new values
  hsize_t memory_len[] = { peak_train->n_peaks };
  hid_t samples_file_dataspace = H5Screate_simple(1, memory_len, NULL);
  if (samples_file_dataspace <= 0) {
    return SET_PEAK_TRAIN_CREATE_SAMPLES_FILE_DATASPACE_FAIL;
  }

  hid_t values_file_dataspace = H5Screate_simple(1, memory_len, NULL);
  if (values_file_dataspace <= 0) {
    return SET_PEAK_TRAIN_CREATE_VALUES_FILE_DATASPACE_FAIL;
  }

  hid_t samples_memory_dataspace = H5Screate_simple(1, memory_len, NULL);
  if (samples_memory_dataspace <= 0) {
    return SET_PEAK_TRAIN_CREATE_SAMPLES_MEMORY_DATASPACE_FAIL;
  }

  hid_t values_memory_dataspace = H5Screate_simple(1, memory_len, NULL);
  if (values_memory_dataspace <= 0) {
    return SET_PEAK_TRAIN_CREATE_VALUES_MEMORY_DATASPACE_FAIL;
  }

  // Create the new datasets
  hid_t new_samples_dataset = H5Dcreate2(phase->fid,
                                 samples_group_str,
                                 H5T_NATIVE_ULONG,
                                 samples_file_dataspace,
                                 H5P_DEFAULT,
                                 H5P_DEFAULT,
                                 H5P_DEFAULT);
  if (new_samples_dataset <= 0) {
    return SET_PEAK_TRAIN_CREATE_SAMPLES_FILE_DATASET_FAIL;
  }

  hid_t new_values_dataset = H5Dcreate2(phase->fid,
                                 values_group_str,
                                 H5T_NATIVE_FLOAT,
                                 values_file_dataspace,
                                 H5P_DEFAULT,
                                 H5P_DEFAULT,
                                 H5P_DEFAULT);
  if (new_values_dataset <= 0) {
    return SET_PEAK_TRAIN_CREATE_VALUES_FILE_DATASET_FAIL;
  }

  // Write the new values
  res = H5Dwrite(new_samples_dataset, H5T_NATIVE_ULONG, samples_file_dataspace, samples_memory_dataspace, H5P_DEFAULT, peak_train->samples);
  if (res < 0) {
    return SET_PEAK_TRAIN_WRITE_SAMPLES_DATASET_FAIL;
  }

  res = H5Dwrite(new_values_dataset, H5T_NATIVE_FLOAT, values_file_dataspace, values_memory_dataspace, H5P_DEFAULT, peak_train->values);
  if (res < 0) {
    return SET_PEAK_TRAIN_WRITE_VALUES_DATASET_FAIL;
  }

  // Close the opened identifiers
  res = H5Sclose(samples_file_dataspace);
  if (res < 0) {
    return SET_PEAK_TRAIN_CLOSE_SAMPLES_FILE_DATASPACE_FAIL;
  }

  res = H5Sclose(values_file_dataspace);
  if (res < 0) {
    return SET_PEAK_TRAIN_CLOSE_VALUES_FILE_DATASPACE_FAIL;
  }

  res = H5Sclose(samples_memory_dataspace);
  if (res < 0) {
    return SET_PEAK_TRAIN_CLOSE_SAMPLES_MEMORY_DATASPACE_FAIL;
  }

  res = H5Sclose(values_memory_dataspace);
  if (res < 0) {
    return SET_PEAK_TRAIN_CLOSE_VALUES_MEMORY_DATASPACE_FAIL;
  }

  res = H5Dclose(new_samples_dataset);
  if (res < 0) {
    return SET_PEAK_TRAIN_CLOSE_SAMPLES_DATASET_FAIL;
  }

  res = H5Dclose(new_values_dataset);
  if (res < 0) {
    return SET_PEAK_TRAIN_CLOSE_VALUES_DATASET_FAIL;
  }

  return OK;
}
