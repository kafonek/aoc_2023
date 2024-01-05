mod day01;
mod day02;
mod day04;
mod day05;
mod day06;
mod day07;
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

    let day05 = PyModule::new(py, "day05")?;
    day05.add_class::<day05::PyPipeline>()?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023_pyo3.day05", day05)?;

    let day06 = PyModule::new(py, "day06")?;
    day06.add_class::<day06::PyRace>()?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023_pyo3.day06", day06)?;

    let day07 = PyModule::new(py, "day07")?;
    day07.add_class::<day07::PyHand>()?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2023_pyo3.day07", day07)?;

    Ok(())
}
