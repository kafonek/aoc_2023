use aoc_2023::day05::Pipeline;
use pyo3::prelude::*;

#[pyclass(name = "Pipeline")]
pub struct PyPipeline {
    inner: Pipeline,
}

#[pymethods]
impl PyPipeline {
    #[staticmethod]
    fn from_lines(lines: Vec<String>) -> Self {
        PyPipeline {
            inner: Pipeline::from_lines(lines),
        }
    }

    #[pyo3(signature = (seed, debug=false))]
    fn get(&self, seed: usize, debug: bool) -> PyResult<usize> {
        Ok(self.inner.get(seed, debug))
    }
}
