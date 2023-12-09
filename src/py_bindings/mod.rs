mod day02;
mod day04;
mod day05;
mod day06;

use pyo3::prelude::*;

#[pymodule]
fn aoc_2023(py: Python, m: &PyModule) -> PyResult<()> {
    let pyo3 = PyModule::new(py, "pyo3")?;
    // Hack from https://github.com/PyO3/pyo3/issues/759#issuecomment-1208179322
    // makes from aoc_2023.pyo3 import Thing work
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023.pyo3", pyo3)?;

    let day02 = PyModule::new(py, "day02")?;
    day02.add_class::<day02::PyBag>()?;
    day02.add_class::<day02::PyGame>()?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023.pyo3.day02", day02)?;

    let day04 = PyModule::new(py, "day04")?;
    day04.add_class::<day04::PyCard>()?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023.pyo3.day04", day04)?;

    m.add_submodule(pyo3)?;

    let day05 = PyModule::new(py, "day05")?;
    day05.add_class::<day05::PyPipeline>()?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023.pyo3.day05", day05)?;

    let day06 = PyModule::new(py, "day06")?;
    day06.add_class::<day06::PyRace>()?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023.pyo3.day06", day06)?;
    Ok(())
}
