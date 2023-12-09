use crate::rust::day06::Race;
use pyo3::prelude::*;

#[pyclass(name = "Race")]
pub struct PyRace {
    inner: Race,
}

#[pymethods]
impl PyRace {
    #[new]
    fn new(duration: usize, distance_to_beat: usize) -> Self {
        PyRace {
            inner: Race::new(duration, distance_to_beat),
        }
    }

    #[getter]
    fn answer(&self) -> PyResult<u32> {
        Ok(self.inner.answer)
    }

    pub fn run(&mut self) {
        self.inner.run();
    }
}
