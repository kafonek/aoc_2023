use aoc_2023::day01::Calibration;
use pyo3::prelude::*;

#[pyclass(name = "Calibration")]
pub struct PyCalibration {
    inner: Calibration,
}

#[pymethods]
impl PyCalibration {
    #[staticmethod]
    fn from_line(line: &str) -> PyResult<Self> {
        Ok(PyCalibration {
            inner: Calibration::from_line(line),
        })
    }

    fn value(&self) -> i32 {
        self.inner.value()
    }

    fn value2(&self) -> i32 {
        self.inner.value2()
    }
}
