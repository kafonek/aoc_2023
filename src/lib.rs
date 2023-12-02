pub mod py_bindings;
pub mod utils;

use crate::py_bindings::day02::{PyBag, PyGame};
use pyo3::prelude::*;

#[pymodule]
fn aoc_2023(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBag>()?;
    m.add_class::<PyGame>()?;
    Ok(())
}
