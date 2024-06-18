use crate::error::SpikeError;

/// PhaseHandler
///
/// Trait for handling a phase recording
/// Any implementer must provide access to the phase information and
/// it's responsible of reading and writing data
#[rustfmt::skip]
pub trait PhaseHandler {
    //--------------------------------------------------------------------------
    // general info
     
    /// Returns the sampling frequency of the recorded data
    fn sampling_frequency(&self) -> f32;
    /// Returns the total number of samples of the recording
    fn datalen(&self) -> usize;
    /// Returns the list of the labels associated to active channels
    fn labels(&self) -> Vec<String>;

    //--------------------------------------------------------------------------
    // raw data
    fn raw_data(&self, channel: &str, start: Option<usize>, end: Option<usize>) -> Result<Vec<f32>, SpikeError>;
    fn set_raw_data(&mut self, channel: &str, start: Option<usize>, end: Option<usize>, data: &[f32]) -> Result<(), SpikeError>;

    //--------------------------------------------------------------------------
    // digital channels
    fn n_digitals(&self) -> usize;
    fn digital(&self, index: usize, start: Option<usize>, end: Option<usize>) -> Result<Vec<f32>, SpikeError>;
    fn set_digital(&mut self, index: usize, start: Option<usize>, end: Option<usize>, data: &[f32]) -> Result<(), SpikeError>;

    //--------------------------------------------------------------------------
    // event channels
    fn n_events(&self) -> usize;
    fn events(&self, index: usize) -> Result<Vec<u64>, SpikeError>;

    //--------------------------------------------------------------------------
    // peak trains
    fn peak_train(&self, channel: &str, start: Option<usize>, end: Option<usize>) -> Result<(Vec<f32>, Vec<usize>), SpikeError>;
    fn set_peak_train(&mut self, channel: &str, start: Option<usize>, end: Option<usize>, data: (Vec<f32>, Vec<usize>)) -> Result<(), SpikeError>;
}
