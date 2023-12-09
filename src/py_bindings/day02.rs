use crate::rust::day02::{Bag, Game};
use pyo3::prelude::*;

#[pyclass(name = "Bag")]
pub struct PyBag {
    inner: Bag,
}

#[pymethods]
impl PyBag {
    #[new]
    fn new(red: i32, green: i32, blue: i32) -> Self {
        PyBag {
            inner: Bag::new(red, green, blue),
        }
    }

    #[staticmethod]
    fn from_string(s: &str) -> Self {
        PyBag {
            inner: Bag::from_string(s).unwrap(),
        }
    }

    fn can_contain(&self, other: &PyBag) -> bool {
        self.inner.can_contain(&other.inner)
    }

    fn power(&self) -> i32 {
        self.inner.power()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }
}

#[pyclass(name = "Game")]
pub struct PyGame {
    inner: Game,
}

#[pymethods]
impl PyGame {
    #[staticmethod]
    fn from_string(s: &str) -> Self {
        PyGame {
            inner: Game::from_string(s).unwrap(),
        }
    }

    #[getter]
    fn id(&self) -> PyResult<i32> {
        Ok(self.inner.id)
    }

    fn check(&self, bag: &PyBag) -> bool {
        self.inner.check(&bag.inner)
    }

    fn max_values(&self) -> PyBag {
        PyBag {
            inner: self.inner.max_values(),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }
}
