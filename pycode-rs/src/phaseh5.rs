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
            None
        }
    }

    pub fn datalen(&self) -> Option<usize> {
        if self._phaseh5.is_some() {
            Some(self._phaseh5.as_ref().unwrap().datalen())
        } else {
            None
        }
    }

    pub fn labels(&self) -> Option<Vec<String>> {
        if self._phaseh5.is_some() {
            Some(self._phaseh5.as_ref().unwrap().labels())
        } else {
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
            match self._phaseh5.as_ref().unwrap().raw_data(channel, start, end) {
                Ok(data) => Some(data),
                Err(err) => {
                    println!("{:?}", err);
                    None
                }
            }
        } else {
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
            if let Ok(()) = self._phaseh5.as_mut().unwrap().set_raw_data(channel, start, &data) {
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }
}
