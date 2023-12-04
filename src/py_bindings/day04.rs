use crate::utils::day04::Card;
use pyo3::prelude::*;

#[pyclass(name = "Card")]
pub struct PyCard {
    inner: Card,
}

#[pymethods]
impl PyCard {
    #[staticmethod]
    fn from_string(s: &str) -> Self {
        PyCard {
            inner: Card::from_string(s),
        }
    }

    #[getter]
    fn id(&self) -> PyResult<i32> {
        Ok(self.inner.id)
    }

    fn score(&self) -> PyResult<i32> {
        Ok(self.inner.score())
    }

    fn copies(&self) -> PyResult<Vec<i32>> {
        Ok(self.inner.copies())
    }
}
