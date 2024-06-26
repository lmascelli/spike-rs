# Pycode

## Types

### PyPhase

```python
class PyPhase:

    # fields
    sampling_frequency: float
    channel_labels: List[str]
    digitals_lengths: List[int]
    raw_data_lengths: Dict[str, int]
    peak_train_lengths: Dict[str, int]
    _valid: bool

    # methods
    def __init__(self, phase: sp.Phase):
        ...

    def from_file(filepath: Path) -> PyPhase:
        ...

    def save(self, filepath: Path):
        ...

    def update(self):
        ...

    def get_digital(self, index: int) -> Optional[List[floatl]]:
        ...

    def get_raw_data(self, label: str) -> Optional[List[float]]:
        ...

    def get_peaks_train(self, label: str) -> Optional[List[int]]:
        ...

    def get_el_stim_intervals(self) -> Optional[List[List[int]]]:
        ...

    def get_peaks_bins(
        self, n_bins: int = 50
    ) -> Dict[str, Tuple[List[int], float, float]]:
        ...
        
    def peak_detection(
            self, peak_duration: float, refractary_time: float, n_devs: float
        ):
        ...

    def get_peaks_in_consecutive_intervals(
        self, intervals: List[Tuple[int, int]]
    ) -> Dict[str, List[List[int]]]:
        ...

    def get_digital_intervals(self, index) -> List[Tuple[int, int]]:
        ...

    def clear_peaks_over_threshold(self, threshold: float):
        ...

    def get_subsampled_pre_stim_post_from_intervals(
        self, intervals: List[Tuple[int, int]], bin_size: int
    ) -> Dict[str, List[Tuple[List[int], List[int], List[int]]]]:
        ...

    def psth(self, bin_size: int, digital_index: int) -> Optional[List[List[int]]]:
        ...
```

### MCExplorer

```python
class MCExplorer:

    # fields
    filename: str
    explorer: sp.MCExplorer

    # methods

    def __init__(self, filename: Path):
        ...

    def list_recordings(self) -> Optional[List[Tuple[int, str]]]:
        ...

    def recording_info(self, recording_index: int) -> Optional[str]:
        ...

    def list_analogs(self, recording_index: int) -> Optional[List[Tuple[int, str]]]:
        ...

    def analog_info(self, recording_index: int, analog_index: int) -> Optional[str]:
        ...

    def analog_dims(self, recording_index: int, analog_index: int) -> Optional[List[int]]:
        ...

    def list_analog_channels(self, recording_index: int, analog_index: int
                             ) -> Optional[List[str]]:
        ...

    def get_channel_data(self, recording_index: int,
                         analog_index: int,
                         channel_label: str) -> Optional[List[float]]:
        ...

    def convert_phase(self, recording_index: int,
                      raw_data_index: int,
                      digital_index: Optional[int],
                      event_index: Optional[int]) -> Optional[PyPhase]:
        ...

    def convert_with_rule(self,
                          rule: Callable[[str], Optional[ConvertingValues]],
                          dest: str):
        ...


    def __str__(self) -> str:
        ...
```

## Operations

## Utils

```python

def convert_mc_acquisition(
    source: Path,
    mcdataconv_path: Path,
    wine_prefix: Optional[str] = None
    ) -> bool:
    ...

def convert_mc_h5_phase(source: Path, dest: Path) -> bool:
    ...
```

---

# Spike-rs
## Core
### Types
```rust
struct Phase {
    sampling_frequency: f32,
    raw_data: HashMap<String, Vec<f32>>,
    peak_trains: HashMap<String, (Vec<f32>, Vec<usize>)>,
    digitals: Vec<Vec<f32>>,
    el_stim_intervals: Vec<Vec<u64>>,
}

impl Phase {
    fn new() -> Phase;

    fn compute_peak_train(&self,
            label: &str,
            peak_duration: f32,
            refractory_time: f32,
            n_devs: f32) -> Option<()>; // TODO

    fn compute_all_peak_trains(&mut self,
            peak_duration: f32,
            refractory_time: f32,
            n_devs: f32,
            ) -> Option<()>;

    fn clear_peaks_over_threshold(&mut self, threshold: f32);

    fn get_peaks_in_interval(&self,
            interval: &(usize, usize)
            ) -> HashMap<String, (Vec<f32>, Vec<usize>)>;

    fn get_peaks_in_consecutive_intervals(&self,
            intervals: &[(usize, usize)]
            ) -> HashMap<String, (Vec<f32>, Vec<usize>)>;

    fn get_subsampled_pre_stim_post_from_intervals(&self,
            intervals: &[(usize, usize)],
            bin_size: usize
            ) -> HashMap<String, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>>;

    fn psth(&self, bin_size: usize, digital_index: usize
           ) -> Result<Vec<Vec<usize>>, String>;
}
```

### Operations 
```rust
fn compute_threshold(range: &[f32],
        sampling_frequency: f32,
        multiplier: f32) -> Result<f32, String>;

fn spike_detection(data: &[f32],
        sampling_frequency: f32,
        threshold: f32,
        peak_duration: f32,
        refractory_time: f32) -> Option<(Vec<f32>, Vec<usize>)>;

fn get_peaks_bins(range: &[f32], n_bins: usize) -> Option<(Vec<usize>, f32, f32)>;

fn get_digital_intervals(digital: &[f32]) -> Vec<(usize, usize)>;

fn subsample_range(peak_times: &[usize],
        starting_sample: usize,
        bin_size: usize,
        n_bins: usize) -> Vec<usize>;
```

#### Math
```rust
fn mean(range: &[f32]) -> f32;

fn stdev(range: &[f32]) -> f32;

fn min(range: &[f32]) -> f32;

fn max(range: &[f32]) -> f32;

fn exp_fit(x: &[f32], y: &[f32]) -> (f32, f32); // TODO
```

---

# Pycode-rs
## PyPhase
```rust
struct PyPhase {
    sampling_frequency: f32,
    channel_labels: Vec<String>,
    raw_data_lengths: HashMap<String, usize>,
    peak_train_lengths: HashMap<String, usize>,
    digitals_lengths: Vec<usize>,
    phase: Phase,
}

impl PyPhase {
    fn new() -> PyPhase;

    fn update(&mut self);

    fn get_digitals(&self, index: usize) -> Option<Vec<f32>>;

    fn get_raw_data(&self, label: &str) -> Option<Vec<f32>>;

    fn get_el_stim_intervals(&self) -> Option<Vec<Vec<u64>>>;

    fn get_peaks_train(&self, label: &str) -> Option<(Vec<f32>, Vec<usize>)>;

    fn compute_all_peak_trains(
            &mut self,
            peak_duration: f32,
            refractory_time: f32,
            n_devs: f32);

    fn clear_peaks_over_threshold(&mut self, threshold: f32);

    fn get_peaks_bins(&self, n_bins: usize) -> HashMap<String, (Vec<usize>, f32, f32)>;

    fn get_digital_intervals(&self, index: usize) -> Option<Vec<(usize, usize)>>;

    fn get_peaks_in_consecutive_intervals(
        &self,
        intervals: Vec<(usize, usize)>,
    ) -> HashMap<String, (Vec<f32>, Vec<usize>)>;

    fn get_peaks_in_interval(
        &self,
        interval: (usize, usize)
    ) -> HashMap<String, (Vec<f32>, Vec<usize>)>;

    fn get_subsampled_pre_stim_post_from_intervals(
        &self,
        intervals: Vec<(usize, usize)>,
        bin_size: usize,
    ) -> HashMap<String, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>>;
    
    fn psth(&self, bin_size: usize, digital_index: usize) -> Option<Vec<Vec<usize>>>;
}

load_phase(filename: &str) -> Option<PyPhase>;

save_phase(phase: &PyPhase, filename: &str) -> bool;
```
## MCExplorer

```rust
struct PyMCExplorer {
    content: Option<H5Content>,
}

impl PyMCExplorer {
    fn new(filename: &str) -> PyMCExplorer;

    fn __str__(&self) -> String;

    fn list_recordings(&self) -> Vec<(usize, String)>;

    fn recording_info(&self, recording_index: usize) -> Option<String>;

    fn list_analogs(&self, recording_index: usize) -> Option<Vec<(usize, String)>>;

    fn analog_info(&self, recording_index: usize, analog_index: usize) -> Option<String>;

    fn analog_dims(&self, recording_index: usize, analog_index: usize) -> Option<Vec<usize>>;

    fn list_analog_channels(
        &self,
        recording_index: usize,
        analog_index: usize,
    ) -> Option<Vec<String>>;

    fn get_channel_data(
        &self,
        recording_index: usize,
        analog_index: usize,
        channel_label: &str,
    ) -> Option<Vec<f32>>;

    fn convert_phase(
        &self,
        recording_index: usize,
        raw_data_index: usize,
        digital_index: Option<usize>,
        event_index: Option<usize>,
    ) -> Option<PyPhase>;
}
```
