#include <hdf5.h>
#define DATE_STRING_LEN 32
#define ANALOG_LABEL_STRING_LEN 64
#define CHANNEL_LABEL_STRING_LEN 32
#define MAX_CHANNELS 60

typedef enum phaseh5_error {
  OK,
  OPEN_FAIL,
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
  OPEN_CHANNEL_DATA_FAIL,
  NO_RAW_DATA_STREAM,
  RAW_DATA_END_BEFORE_START,
  RAW_DATA_END_OUT_OF_BOUNDS,
  RAW_DATA_GET_DATASPACE_FAIL,
  RAW_DATA_SELECT_HYPERSLAB_FAIL,
  RAW_DATA_CREATE_MEMORY_DATASPACE_FAIL,
  RAW_DATA_READ_DATA_FAIL,
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
  // InfoChannel data
  InfoChannel info_channels[MAX_CHANNELS];
} AnalogStream;

/* phaseh5_error open_analog(AnalogStream* analog_stream, hid_t analog_stream_group); */
/* phaseh5_error close_analog(AnalogStream* analog_stream); */

typedef struct PhaseH5 {
  hid_t fid;
  char date[DATE_STRING_LEN];
  size_t datalen;
  float sampling_frequency;
  AnalogStream raw_data;
  bool has_digital;
  AnalogStream digital;
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

/*

 */
phaseh5_error raw_data(PhaseH5* phase, size_t index, size_t start, size_t end, float* buf);
