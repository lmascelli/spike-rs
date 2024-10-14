#include <hdf5.h>
#define DATE_STRING_LEN 32
#define ANALOG_LABEL_STRING_LEN 64
#define CHANNEL_LABEL_STRING_LEN 32
#define MAX_EVENT_STREAMS 16
#define MAX_CHANNELS 60


typedef enum phaseh5_error {
  OK,
  OPEN_FAIL,
  CLOSE_FILE_FAIL,
  OPEN_DATA_GROUP_FAIL,
  OPEN_DATE_ATTRIBUTE_FAIL,
  READ_DATE_ATTRIBUTE_FAIL,
  OPEN_DATE_DATATYPE_FAIL,
  OPEN_ANALOG_GROUP_FAIL,
  OPEN_INFO_CHANNEL_DATASET_FAIL,
  OPEN_INFO_CHANNEL_DATASPACE_FAIL,
  OPEN_INFO_CHANNEL_DATATYPE_FAIL,
  OPEN_ANALOG_DATASET_FAIL,
  OPEN_LABEL_ATTRIBUTE_FAIL,
  READ_LABEL_ATTRIBUTE_FAIL,
  OPEN_LABEL_DATATYPE_FAIL,
  READ_INFO_CHANNELS_FAIL,
  PARSE_ANALOG_STREAM_DIFFERENT_TICK,
  MULTIPLE_DIGITAL_STREAMS,
  MULTIPLE_RAW_DATA_STREAMS,
  MULTIPLE_SAMPLING_FREQUENCIES,
  MULTIPLE_DATALENS,
  OPEN_CHANNEL_DATA_FAIL,
  OPEN_CHANNEL_DATA_DATASPACE_FAIL,
  GET_CHANNEL_DATA_DIMS_FAIL,
  NO_RAW_DATA_STREAM,
  OPEN_EVENT_STREAM_GROUP_LINK_FAIL,
  OPEN_EVENT_STREAM_GROUP_FAIL,
  OPEN_EVENT_STREAM_STREAM_0_GROUP_LINK_FAIL,
  MAX_EVENT_STREAMS_EXCEEDED,
  OPEN_ENTITY_DATASET_FAIL,
  EVENT_ENTITY_DATASET_CLOSE_FAIL,
  OPEN_PEAK_TRAIN_GROUP_FAIL,
  CREATE_PEAK_GROUP_FAIL,
  RAW_DATA_END_BEFORE_START,
  RAW_DATA_END_OUT_OF_BOUNDS,
  RAW_DATA_GET_DATASPACE_FAIL,
  RAW_DATA_SELECT_HYPERSLAB_FAIL,
  RAW_DATA_CREATE_MEMORY_DATASPACE_FAIL,
  RAW_DATA_READ_DATA_FAIL,
  SET_RAW_DATA_END_BEFORE_START,
  SET_RAW_DATA_END_OUT_OF_BOUNDS,
  SET_RAW_DATA_GET_DATASPACE_FAIL,
  SET_RAW_DATA_SELECT_HYPERSLAB_FAIL,
  SET_RAW_DATA_CREATE_MEMORY_DATASPACE_FAIL,
  SET_RAW_DATA_WRITE_DATASET_FAIL,
  DIGITAL_NO_DIGITAL,
  DIGITAL_END_BEFORE_START,
  DIGITAL_END_OUT_OF_BOUNDS,
  DIGITAL_GET_DATASPACE_FAIL,
  DIGITAL_SELECT_HYPERSLAB_FAIL,
  DIGITAL_CREATE_MEMORY_DATASPACE_FAIL,
  DIGITAL_READ_DATA_FAIL,
  SET_DIGITAL_NO_DIGITAL,
  SET_DIGITAL_END_BEFORE_START,
  SET_DIGITAL_END_OUT_OF_BOUNDS,
  SET_DIGITAL_GET_DATASPACE_FAIL,
  SET_DIGITAL_SELECT_HYPERSLAB_FAIL,
  SET_DIGITAL_CREATE_MEMORY_DATASPACE_FAIL,
  SET_DIGITAL_WRITE_DATA_FAIL,
  EVENTS_LEN_INDEX_OUT_OF_BOUNDS,
  EVENTS_LEN_OPEN_EVENT_DATASPACE_FAIL,
  EVENTS_INDEX_OUT_OF_BOUNDS,
  EVENTS_LEN_GET_DIMS_FAIL,
  EVENTS_GET_EVENTS_DATASPACE_FAIL,
  EVENTS_SELECT_DATASPACE_HYPERSLAB_FAIL,
  EVENTS_CREATE_MEMORY_DATASPACE_FAIL,
  EVENTS_READ_DATASET_FAIL,
  PEAK_TRAIN_NO_PEAK_GROUP,
  PEAK_TRAIN_VALUES_DATASET_LINK_FAIL,
  PEAK_TRAIN_NO_VALUES_DATASET,
  PEAK_TRAIN_SAMPLES_DATASET_LINK_FAIL,
  PEAK_TRAIN_NO_SAMPLES_DATASET,
  PEAK_TRAIN_OPEN_VALUES_DATASET_FAIL,
  PEAK_TRAIN_OPEN_SAMPLES_DATASET_FAIL,
  DELETE_PEAK_TRAIN_VALUES_DATASET_LINK_FAIL,
  DELETE_PEAK_TRAIN_NO_VALUES_DATASET,
  DELETE_PEAK_TRAIN_SAMPLES_DATASET_LINK_FAIL,
  DELETE_PEAK_TRAIN_NO_SAMPLES_DATASET,
  DELETE_PEAK_TRAIN_VALUES_DATASET_FAIL,
  DELETE_PEAK_TRAIN_SAMPLES_DATASET_FAIL,
  PEAK_TRAIN_LEN_OPEN_VALUES_DATASPACE_FAIL,
  PEAK_TRAIN_LEN_GET_VALUES_DATASPACE_DIM_FAIL,
  PEAK_TRAIN_LEN_OPEN_SAMPLES_DATASPACE_FAIL,
  PEAK_TRAIN_LEN_GET_SAMPLES_DATASPACE_DIM_FAIL,
  PEAK_TRAIN_LEN_VALUES_SAMPLES_DIFFERENT,
  PEAK_TRAIN_LEN_CLOSE_VALUES_DATASET_FAIL,
  PEAK_TRAIN_LEN_CLOSE_SAMPLES_DATASET_FAIL,
  PEAK_TRAIN_CREATE_MEMORY_DATASPACE_FAIL,
  PEAK_TRAIN_READ_VALUES_DATASET_FAIL,
  PEAK_TRAIN_READ_SAMPLES_DATASET_FAIL,
  SET_PEAK_TRAIN_CREATE_SAMPLES_MEMORY_DATASPACE_FAIL,
  SET_PEAK_TRAIN_CREATE_VALUES_MEMORY_DATASPACE_FAIL,
  SET_PEAK_TRAIN_CREATE_SAMPLES_MEMORY_DATASET_FAIL,
  SET_PEAK_TRAIN_CREATE_VALUES_MEMORY_DATASET_FAIL,
} phaseh5_error;

typedef struct InfoChannel {
  int channel_id;
  int row_index;
  int group_id;
  int electrode_group;
  const char *label;
  const char *raw_data_type;
  const char *unit;
  int exponent;
  int ad_zero;
  long int tick;
  long int conversion_factor;
  int adc_bits;
  const char *high_pass_filter_type;
  const char *high_pass_filter_cutoff;
  int high_pass_filter_order;
  const char *low_pass_filter_type;
  const char *low_pass_filter_cutoff;
  int low_pass_filter_order;
} InfoChannel;

typedef struct AnalogStream {
  const char label[ANALOG_LABEL_STRING_LEN];
  hsize_t n_channels;
  // ChannelData dataset
  hid_t channel_data_dataset;
  size_t datalen;
  // InfoChannel data
  InfoChannel info_channels[MAX_CHANNELS];
} AnalogStream;

typedef struct PeakTrain {
  size_t n_peaks;
  float* values;
  long int* samples;
} PeakTrain;

typedef struct PhaseH5 {
  hid_t fid;
  char date[DATE_STRING_LEN];
  size_t datalen;
  float sampling_frequency;
  AnalogStream raw_data;
  bool has_digital;
  AnalogStream digital;
  int n_events;
  hid_t event_entities[MAX_EVENT_STREAMS];
  hid_t peaks_group;
} PhaseH5;

/*
  Initialize the library creating the needed custom datatypes.
  To be called at the start of its use.
 */
void pycodeh5_init();

/*
  Finalize the library deleting the created handles.
  To bel called at the end of its use.
 */
void pycodeh5_close();

/*
  Clear the fields of a PhaseH5 struct
 */
void init_phase(PhaseH5* phase);
/*
  Open a Phase from a .h5 file and parse its content
 */
phaseh5_error phase_open(PhaseH5* phase, const char *filename);
/*
  Close a PhaseH5 clearing the allocated resources
 */
phaseh5_error phase_close(PhaseH5* phase);

phaseh5_error raw_data(PhaseH5* phase, size_t index, size_t start, size_t end, int* buf);
phaseh5_error set_raw_data(PhaseH5* phase, size_t index, size_t start, size_t end, int* buf);
phaseh5_error digital(PhaseH5* phase, size_t start, size_t end, int* buf);
phaseh5_error set_digital(PhaseH5* phase, size_t start, size_t end, int* buf);
phaseh5_error events_len(PhaseH5* phase, size_t index, hsize_t *len);
phaseh5_error events(PhaseH5* phase, size_t index, long int *buf);
phaseh5_error peak_train_len(PhaseH5*, const char* label, size_t *len);
phaseh5_error peak_train(PhaseH5* phase, const char* label, PeakTrain* peak_train);
phaseh5_error set_peak_train(PhaseH5* phase, const char* label, PeakTrain* peak_train);
