use crate::error::SpikeError;

/// PhaseHandler
///
/// Trait for handling a phase recording
/// Any implementer must provide access to the phase information and
/// it's responsible of reading and writing data
pub trait PhaseHandler {
    //--------------------------------------------------------------------------
    // GENERAL INFO

    /// Returns the sampling frequency of the recorded data
    fn sampling_frequency(&self) -> f32;

    /// Returns the total number of samples of the recording
    fn datalen(&self) -> usize;

    /// Returns the list of the labels associated to active channels
    fn labels(&self) -> Vec<String>;

    //--------------------------------------------------------------------------
    // RAW DATA

    /// Returns a slice of the raw data of the selected channel, if exists.
    /// The `start` and `end` of the slice are optional and if omitted the
    /// start and the end of the whole data will be used.
    fn raw_data(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<Vec<f32>, SpikeError>;

    /// Replace a slice of the raw data of the selected channel, if exists
    /// with the provided `data`.
    /// The `start` and `end` of the slice are optional and if omitted the
    /// start and the end of the whole data will be used.
    fn set_raw_data(
        &mut self,
        channel: &str,
        start: Option<usize>,
        data: &[f32],
    ) -> Result<(), SpikeError>;

    //--------------------------------------------------------------------------
    // DIGITAL CHANNELS

    /// Returns the number of digital channels stored in the recording.
    fn n_digitals(&self) -> usize;

    /// Returns a slice of the digital channel selected, if exists.
    /// The `start` and `end` of the slice are optional and if omitted the
    /// start and the end of the whole data will be used.
    fn digital(
        &self,
        index: usize,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<Vec<f32>, SpikeError>;

    /// Replace a slice of the digital channel selected, if exists, with
    /// the provided `data`.
    /// The `start` and `end` of the slice are optional and if omitted the
    /// start and the end of the whole data will be used.
    fn set_digital(
        &mut self,
        index: usize,
        start: Option<usize>,
        data: &[f32],
    ) -> Result<(), SpikeError>;

    //--------------------------------------------------------------------------
    // EVENT CHANNELS

    /// Returns the number of events arrays contained in the recording
    fn n_events(&self) -> usize;

    /// Returns the selected events array, if exists.
    fn events(&self, index: usize) -> Result<Vec<i64>, SpikeError>;

    //--------------------------------------------------------------------------
    // PEAK TRAINS

    /// Returns a slice of the peak trains of the selected channel, if exists.
    /// The `start` and `end` of the slice are optional and if omitted the
    /// start and the end of the whole data will be used. The train is supposed
    /// to be sorted in time.
    fn peak_train(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<(Vec<usize>, Vec<f32>), SpikeError>;

    /// Replace a slice of the peak trains of the selected channel, if exists
    /// with the provided `data`.
    /// The `start` and `end` of the slice are optional and if omitted the
    /// start and the end of the whole data will be used.
    /// THE TRAIN IS SUPPOSED TO BE SORTED IN TIME AND THE PROVIDED `data` 
    /// MUST BE VALID IN THE REPLACED SLICE. PAY ATTENTION THAT THERE IS NO
    /// CHECK ON THAT FOR NOW!!!
    fn set_peak_train(
        &mut self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
        data: (Vec<usize>, Vec<f32>),
    ) -> Result<(), SpikeError>;
}
