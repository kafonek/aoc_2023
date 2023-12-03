# aoc_2023

Advent of Code 2023 - Rust + Python

# Project Layout

```
/data
  day01.txt <-- "prod" problem data for @kafonek AoC account
  day02.txt
  ...

/notebooks
  day02-1-py.ipynb <-- prototype and solve "prod" problem with %%timeit
  day02-1-rs.ipynb <-- prototype Rust here, no full solution
  day02-1-pyo3.ipynb <-- if there's Rust code for the problem, Python nb equivalent using pyo3 bindings
  ...

/src
  lib.rs
  bin/
    day01-1.rs <-- Rust solution for "prod" problem with timing
    ...
  utils/ 
    mod.rs
    day02.rs <-- For situations where structs/fn's are shared between parts 1 and 2 
    ...
  py_bindings/
    day02.rs <-- pyo3 code wrapping the "pure Rust" code from utils/day02.rs

Cargo.toml <-- Rust deps
pyproject.toml <-- Python deps
```

# Notebooks

- `poetry install` should install `jupyter`, `notebook`, and `ipykernel` so that basic `-py` Notebooks work
- [setup Rust](https://www.rust-lang.org/tools/install) and [jupyter-evcxr](https://crates.io/crates/evcxr_jupyter) to enable a Rust kernel to run the `-rs` Notebooks
- `maturin develop --release` will install the `aoc_2023` Python package, which are `pyo3` bindings to Rust code, and used in the `-pyo3` Notebooks

To start the server: `poetry run jupyter notebook` (or `poetry run jupyter lab` if you prefer Jupyter Lab UI)

## Jupyter-evcxr

To use third-party crates on the Rust kernel, include `:deps` in a cell. Evcxr will cache the download but not the compilation step by default, so using `:deps` is a bit slow. There is an option for [caching compilation in the evcxr documentation](https://github.com/evcxr/evcxr/blob/main/COMMON.md#caching) but it didn't seem to work for me and there are [various](https://github.com/evcxr/evcxr/issues/218) [github](https://github.com/evcxr/evcxr/issues/304) [issues](https://github.com/evcxr/evcxr/issues/319) on the topic. 

# Pyo3 Bindings

One purpose of this repo is to explore how to create and utilize Python bindings to Rust code which can potentially improve performance. [pyo3](https://pyo3.rs/v0.14.5/) is used within the Rust code to "wrap" Rust code for Python usage, while [maturin](https://github.com/PyO3/maturin) is the build tool that can create Python wheels so that wrapped code is importable.

- `Cargo.toml` includes the `pyo3` dependency and has special instructions to build the `.so` file (`cdylib` in addition to `rlib`)
- `pyproject.toml` includes `maturin` as a dependency and specifies using `maturin` in its `build-system`
- Compare the `%%timeit` results between `-py` and `-pyo3` Notebooks to see how effective Python bindings to Rust code can be

# Problem Notes

Thoughts about each days problem, and timing from the pure Rust solutions.

## Day 01
 - Part 2 was quite gimmicky imo with its wrinkle that spelled-out numbers could overlap in letters, like "eightwo" would match 8 and 2. Creating a regex pattern like "one|two|three|..." and doing normal match-iteration didn't work because regex by default consumes the longest match. To get around that, used a hacky "find a match then start looking again from match start index + 1" approach.

 ```
 ❯ cargo run --bin day01-1 --release
Reading data from: "./data/day01.txt"
Answer: 55017
Time: 474.444µss

❯ cargo run --bin day01-2 --release
Reading data from: "./data/day01.txt"
Answer: 53539
Time: 828.054µs
```

## Day 02
 - Much easier than the day 01 part 2
 - Really liked the symmetry between the Python and Rust code in this problem

```
❯ cargo run --bin day02-1 --release
Reading data from: "./data/day02.txt"
Answer: 2795
Time: 130.991µs

❯ cargo run --bin day02-2 --release
Reading data from: "./data/day02.txt"
Answer: 75561
Time: 128.713µs
```

## Day 03
 - classic [gridthings](https://github.com/kafonek/gridthings) stuff, working on [porting that to rust](https://github.com/kafonek/gridthings-rs) now
 - no RS notebook until I get `gridthings-rs` up onto crates.io, can't `:dep` a local or github-hosted crate
 - need more time to write pyo3 bindings for `gridthings-rs`
 - pretty amazing to see my Python gridthings implementation take over 1s and Rust take under 1ms

 ```
 ❯ cargo run --bin day03-1 --release
Reading data from: "./data/day03.txt"
Answer: 527144
Time: 847.469µs

❯ cargo run --bin day03-2 --release
Reading data from: "./data/day03.txt"
Answer: 81463996
Time: 1.022036ms
```