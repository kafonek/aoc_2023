use aoc_2023::day06::Race;
use pyo3::prelude::*;

#[pyclass(name = "Race")]
pub struct PyRace {
    inner: Race,
}

#[pymethods]
impl PyRace {
    #[new]
    fn new(time: usize, distance_to_beat: usize) -> Self {
        PyRace {
            inner: Race::new(time, distance_to_beat),
        }
    }

    pub fn run(&self) -> PyResult<usize> {
        Ok(self.inner.run())
    }
}
