use pyo3::prelude::*;
use spike_h5::PhaseH5;
use spike_rs::types::PhaseHandler;

#[pyclass]
pub struct PyPhaseH5 {
    _phaseh5: Option<PhaseH5>,
}

#[pymethods]
impl PyPhaseH5 {
    #[new]
    pub fn open(filename: &str) -> Self {
        Self {
            _phaseh5: if let Ok(_phaseh5) = PhaseH5::open(filename) {
                Some(_phaseh5)
            } else {
                eprintln!("PyCode-rs: ERROR Phase not present");
                None
            },
        }
    }

    pub fn __str__(&self) -> String {
        format!("{:?}", self._phaseh5)
    }

    pub fn sampling_frequency(&self) -> Option<f32> {
        if self._phaseh5.is_some() {
            Some(self._phaseh5.as_ref().unwrap().sampling_frequency())
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            None
        }
    }

    pub fn datalen(&self) -> Option<usize> {
        if self._phaseh5.is_some() {
            Some(self._phaseh5.as_ref().unwrap().datalen())
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            None
        }
    }

    pub fn labels(&self) -> Option<Vec<String>> {
        if self._phaseh5.is_some() {
            Some(self._phaseh5.as_ref().unwrap().labels())
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            None
        }
    }

    #[pyo3(signature = (channel, start=None, end=None))]
    pub fn raw_data(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Option<Vec<f32>> {
        if self._phaseh5.is_some() {
            match self._phaseh5.as_ref().unwrap().raw_data(channel, start, end)
            {
                Ok(data) => Some(data),
                Err(err) => {
                    eprintln!("PyCode-rs: ERROR {:?}", err);
                    None
                }
            }
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            None
        }
    }

    #[pyo3(signature = (channel, data, start=None))]
    pub fn set_raw_data(
        &mut self,
        channel: &str,
        data: Vec<f32>,
        start: Option<usize>,
    ) -> Option<()> {
        if self._phaseh5.is_some() {
            match self
                ._phaseh5
                .as_mut()
                .unwrap()
                .set_raw_data(channel, start, &data)
            {
                Ok(_) => Some(()),
                Err(err) => {
                    eprintln!("PyCode-rs: ERROR {:?}", err);
                    None
                }
            }
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            None
        }
    }

    #[pyo3(signature = ())]
    pub fn n_digitals(&self) -> Option<usize> {
        if self._phaseh5.is_some() {
            Some(self._phaseh5.as_ref().unwrap().n_digitals())
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            None
        }
    }

    #[pyo3(signature = (index, start=None, end=None))]
    pub fn digital(
        &self,
        index: usize,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Option<Vec<f32>> {
        if self._phaseh5.is_some() {
            match self._phaseh5.as_ref().unwrap().digital(index, start, end) {
                Ok(data) => Some(data),
                Err(err) => {
                    eprintln!("PyCode-rs: ERROR {:?}", err);
                    None
                }
            }
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            None
        }
    }

    #[pyo3(signature = (index, data, start=None))]
    pub fn set_digital(
        &mut self,
        index: usize,
        data: Vec<f32>,
        start: Option<usize>,
    ) -> bool {
        if self._phaseh5.is_some() {
            match self._phaseh5.as_mut().unwrap().set_digital(
                index,
                start,
                &data[..],
            ) {
                Ok(_) => true,
                Err(err) => {
                    eprintln!("PyCode-rs: ERROR {:?}", err);
                    false
                }
            }
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            false
        }
    }

    fn n_events(&self) -> Option<usize> {
        if self._phaseh5.is_some() {
            Some(self._phaseh5.as_ref().unwrap().n_events())
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            None
        }
    }

    fn events(&self, index: usize) -> Option<Vec<u64>> {
        if self._phaseh5.is_some() {
            if let Ok(ret) = self._phaseh5.as_ref().unwrap().events(index) {
                Some(ret)
            } else {
                None
            }
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            None
        }
    }

    #[pyo3(signature = (channel, start=None, end=None))]
    fn peak_train(
        &self,
        channel: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Option<(Vec<usize>, Vec<f32>)> {
        if self._phaseh5.is_some() {
            match self
                ._phaseh5
                .as_ref()
                .unwrap()
                .peak_train(channel, start, end)
            {
                Ok(data) => Some(data),
                Err(err) => {
                    eprintln!("PyCode-rs: ERROR {:?}", err);
                    None
                }
            }
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            None
        }
    }

    #[pyo3(signature = (channel, data, start=None, end=None))]
    fn set_peak_train(
        &mut self,
        channel: &str,
        data: (Vec<usize>, Vec<f32>),
        start: Option<usize>,
        end: Option<usize>,
    ) -> bool {
        if self._phaseh5.is_some() {
            match self
                ._phaseh5
                .as_mut()
                .unwrap()
                .set_peak_train(channel, start, end, data)
            {
                Ok(_) => true,
                Err(err) => {
                    eprintln!("PyCode-rs: ERROR {:?}", err);
                    false
                }
            }
        } else {
            eprintln!("PyCode-rs: ERROR Phase not present");
            false
        }
    }
}
