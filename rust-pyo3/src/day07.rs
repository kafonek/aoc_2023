use aoc_2023::day07::Hand;
use pyo3::{prelude::*, pyclass::CompareOp};

#[pyclass(name = "Hand")]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PyHand {
    inner: Hand,
}

#[pymethods]
impl PyHand {
    #[staticmethod]
    fn from_string(s: String) -> PyResult<Self> {
        match Hand::from_string(s) {
            Some(hand) => Ok(PyHand { inner: hand }),
            None => Err(pyo3::exceptions::PyValueError::new_err("Invalid hand")),
        }
    }

    #[getter]
    fn bid(&self) -> PyResult<usize> {
        Ok(self.inner.bid)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        op.matches(self.cmp(other))
    }
}
