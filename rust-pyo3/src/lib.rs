mod day01;
use pyo3::prelude::*;

#[pymodule]
fn aoc_2023_pyo3(py: Python, m: &PyModule) -> PyResult<()> {
    let day01 = PyModule::new(py, "day01")?;
    day01.add_class::<day01::PyCalibration>()?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023_pyo3.day01", day01)?;
    m.add_submodule(day01)?;
    Ok(())
}
