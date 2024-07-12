#[derive(Debug)]
pub enum SpikeError {
    LabelNotFound,
    IndexOutOfRange,
    ReplaceRangeError,
    ComputeThresholdTooFewSamples(usize, usize),
    SpikeDetectionTooFewSamples,
    NoSpikeTrainsAvailable,
    OperationFailed,
}

impl std::fmt::Display for SpikeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)?;
        Ok(())
    }
}

impl std::error::Error for SpikeError {}
