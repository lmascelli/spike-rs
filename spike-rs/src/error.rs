#[derive(Debug)]
pub enum SpikeError {
    Implementation(String),
    RawDataStartIsAfterEnd,
    RawDataLabelNotFound,
    RawDataOutOfBounds,
    SetRawDataOutOfBounds,
    SetRawDataLabelNotFound,
    DigitalNoDigitalPresent,
    DigitalStartIsAfterEnd,
    IndexOutOfRange,
    ReplaceRangeError,

    ComputeThresholdTooFewSamples(usize, usize),

    SpikeDetectionTooFewSamples,
    NoSpikeTrainsAvailable,
    OperationFailed,

    LogISITooFewSamples,
    LogISICalcThresholdNoIntraIndex,
    LogISICalcThresholdIntraAtEndOfPeaks,
    LogISICalcThresholdNoMinWithRequiredVoidParameter,
    LogISIFindBurstTooManyBursts,
}

impl std::fmt::Display for SpikeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)?;
        Ok(())
    }
}

impl std::error::Error for SpikeError {}
