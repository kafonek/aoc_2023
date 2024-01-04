mod day01;
mod day02;
mod day04;
use pyo3::prelude::*;

#[pymodule]
fn aoc_2023_pyo3(py: Python, m: &PyModule) -> PyResult<()> {
    let day01 = PyModule::new(py, "day01")?;
    day01.add_class::<day01::PyCalibration>()?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023_pyo3.day01", day01)?;
    m.add_submodule(day01)?;

    let day02 = PyModule::new(py, "day02")?;
    day02.add_class::<day02::PyBag>()?;
    day02.add_class::<day02::PyGame>()?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023_pyo3.day02", day02)?;
    m.add_submodule(day02)?;

    let day04 = PyModule::new(py, "day04")?;
    day04.add_class::<day04::PyCard>()?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023_pyo3.day04", day04)?;

    Ok(())
}
