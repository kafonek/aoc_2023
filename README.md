# AOC 2023

My goal during Advent of Code 2023 is to work with Python and Rust to get as far through the challenges as possible. My workflow is to prototype a solution in Python, write equivalent Rust code for better performance, and experiment with Python bindings to the Rust code using `pyo3` and `maturin`. At the end, I hope to have great examples to compare and contrast between those three approaches: pure Python, pure Rust, and mixed Python/Rust code. 

I start each problem with a new Python Notebook, where I paste in the "training" data from the problem description. Typically I'll write one or more classes using `dataclass` to represent parts of the problem, with `from_string` class methods for parsing the input text. That structure maps well to Rust code, and in many cases I can use AI tools like Copilot or ChatGPT to "translate this Python code to Rust" with good results.

Once I've solved the "training" problem, I try to apply that to the "production" data to get through part 1 of the problem. After seeing part 2, I begin moving code that is shared between part 1 and part 2 out of the Notebook and into library code. I update the part 1 Notebook to use that library code, and prototype a solution for part 2.

If I have a good understanding of the problem using Notebooks, whether I got a solution or not, I switch to writing something I can use in timing benchmarks over in `src/bin` along with writing the pure Rust and `pyo3` Python bindings to Rust code.

Some problems, like the `day01` puzzle that mostly deals with `regex`, may not have a clear use for Rust code and Python bindings. In that case, I'll skip a `pyo3` solution.

# Project Layout

The project is laid out following the [Cargo Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html) and [maturin mixed Rust/Python](https://github.com/PyO3/maturin/tree/main?tab=readme-ov-file#mixed-rustpython-projects) documentation. In order to run the solution scripts, take the following setup steps first:

 - Install Python (I use [pyenv](https://github.com/pyenv/pyenv) myself)
 - [Install Poetry](https://python-poetry.org/docs/#installation)
 - [Install Rust](https://www.rust-lang.org/tools/install)
 - Set up Python environment, `poetry install`
 - Build Rust lib, `cargo build --release` (sanity check: look for `target/release/libaoc_2023.rlib` and `.so`)
 - Build mixed Python/Rust library, `maturin develop --release` (sanity check: look for `python/aoc_2023/*.so`)
 - Run a solution, e.g. `cargo run --bin day01-1` or `poetry run python src/bin/day01-01.py`

```
data/
  day01.txt <-- datasets for each days problem for @kafonek AoC account
  ...

notebooks/
  day01-1-py.ipynb <-- prototype code that may be moved to python/aoc_2023/day01.py
  day01-1-rs.ipynb <-- prototype code that may be moved to src/rust/day01.rs
  day01-1-pyo3.ipynb <-- prototype Python code that's using pyo3 bindings to Rust
  ...

python/
  aoc_2023/
    __init__.py <-- imports pyo3 namespace to bring it into scope
    aoc_2023.cpython-311-x86_64-linux-gnu.so <-- created by `maturin develop`, git ignored
    day01.py <-- pure Python functions/classes used for each day
    utils.py <-- `run_and_time` to measure execution without print statements slowing things down
    pyo3/
      day01.pyi <-- type hinting for the pyo3 wrapped Rust code

src/
  lib.rs <-- `pub mod` the pure Rust so binary scripts can ues them, `mod` the py_bindings
  utils.rs <-- `run_and_time` to measure execution without print statements slowing things down
  bin/
    day01-1.rs <-- Pure Rust solution that prints out answer + timing
    day01-1.py <-- Pure Python solution that prints out answer + timing
    day01-1-pyo3.py <-- Mixed Rust/Python solution that prints out answer + timing
    ...
  rust/ 
    mod.rs
    day01.rs <-- Pure Rust structs / fn
    ...
  py_bindings/
    mod.rs <-- Has the `#[pymodule]` macro that pyo3 / maturin use to build the .so for Python
    day01.rs <-- `#[pyclass]` / `#[pymethod]` macro wrappers to pure Rust structs
    ...
  

Cargo.toml <-- Rust deps, [lib] section is important for building for Python and Rust at same time
pyproject.toml <-- Python deps and `maturin` build configuration
rustfmt.toml <-- for `cargo +nightly fmt`, I like the "Module" style import formatting
```


# Notebooks

- `poetry install` should install `jupyter`, `notebook`, and `ipykernel` so that basic `-py` Notebooks work
- [setup Rust](https://www.rust-lang.org/tools/install) and [jupyter-evcxr](https://crates.io/crates/evcxr_jupyter) to enable a Rust kernel to run the `-rs` Notebooks
- `maturin develop --release` will install the `aoc_2023` Python package, which are `pyo3` bindings to Rust code, and used in the `-pyo3` Notebooks

To start the server: `poetry run jupyter notebook` (or `poetry run jupyter lab` if you prefer Jupyter Lab UI)

## Jupyter-evcxr

To use third-party crates on the Rust kernel, include `:deps` in a cell. Evcxr will cache the download but not the compilation step by default, so using `:deps` is a bit slow. There is an option for [caching compilation in the evcxr documentation](https://github.com/evcxr/evcxr/blob/main/COMMON.md#caching) but it didn't seem to work for me and there are [various](https://github.com/evcxr/evcxr/issues/218) [github](https://github.com/evcxr/evcxr/issues/304) [issues](https://github.com/evcxr/evcxr/issues/319) on the topic. 



# Problem Notes

In the section below, I'll add my thoughts about interesting parts of the puzzles, roadblocks I ran into, and timing from my desktop running the solutions in `src/bin`.

## Day 01
 - Part 2 was quite gimmicky imo with its wrinkle that spelled-out numbers could overlap in letters, like "eightwo" would match 8 and 2. Creating a regex pattern like "one|two|three|..." and doing normal match-iteration didn't work because regex by default consumes the longest match. To get around that, used a hacky "find a match then start looking again from match start index + 1" approach.

 ```
 ❯ poetry run python src/bin/day01-1.py
Reading data from: data/day01.txt
Answer: 55017
Time: 1.26 ms

❯ cargo run --bin day01-1 --release -q
Reading data from: "data/day01.txt"
Answer: 55017
Time: 1.565645ms


❯ poetry run python src/bin/day01-2.py
Reading data from: data/day01.txt
Answer: 53539
Time: 3.06 ms

❯ cargo run --bin day01-2 --release -q
Reading data from: "data/day01.txt"
Answer: 53539
Time: 869.482µs
```

## Day 02
 - Much easier than the day 01 part 2
 - Really liked the symmetry between the Python and Rust code in this problem

```
❯ poetry run python src/bin/day02-1.py
Reading data from: data/day02.txt
Answer: 2795
Time: 795.98 μs

❯ cargo run --bin day02-1 --release -q
Reading data from: "data/day02.txt"
Answer: 2795
Time: 113.784µs

❯ poetry run python src/bin/day02-1-pyo3.py
Reading data from: data/day02.txt
Answer: 2795
Time: 179.43 μs


❯ poetry run python src/bin/day02-2.py
Reading data from: data/day02.txt
Answer: 75561
Time: 1.02 ms

❯ cargo run --bin day02-2 --release -q
Reading data from: "data/day02.txt"
Answer: 75561
Time: 114.833µs

❯ poetry run python src/bin/day02-2-pyo3.py
Reading data from: data/day02.txt
Answer: 75561
Time: 220.15 μs
```

## Day 03
 - classic [gridthings](https://github.com/kafonek/gridthings) stuff, working on [porting that to rust](https://github.com/kafonek/gridthings-rs) now
 - no RS notebook until I get `gridthings-rs` up onto crates.io, can't `:dep` a local or github-hosted crate
 - need more time to write pyo3 bindings for `gridthings-rs`, so no `-pyo3` solutions yet
 - pretty amazing to see my Python gridthings implementation take over 1s and Rust take under 1ms

 ```
 ❯ poetry run python src/bin/day03-1.py
Reading data from: data/day03.txt
Answer: 527144
Time: 1.81 s

❯ cargo run --bin day03-1 --release -q
Reading data from: "data/day03.txt"
Answer: 527144
Time: 822.297µs


❯ poetry run python src/bin/day03-2.py
Reading data from: data/day03.txt
Answer: 81463996
Time: 1.80 s

❯ cargo run --bin day03-2 --release -q
Reading data from: "./data/day03.txt"
Answer: 81463996
Time: 969.353µs
```

## Day 04
 - Are even days rest days?
 - Another easy example of `dataclass` -> Rust code -> clean pyo3 bindings

```
❯ poetry run python src/bin/day04-1.py
Reading data from: data/day04.txt
Answer: 20855
Time: 982.97 μs

❯ cargo run --bin day04-1 --release -q
Reading data from: "./data/day04.txt"
Answer: 20855
Time: 150.729µs

❯ poetry run python src/bin/day04-1-pyo3.py
Reading data from: data/day04.txt
Answer: 20855
Time: 242.06 μs


❯ poetry run python src/bin/day04-2.py
Reading data from: data/day04.txt
Answer: 5489600
Time: 1.15 ms

❯ cargo run --bin day04-2 --release -q
Reading data from: "./data/day04.txt"
Answer: 5489600
Time: 162.951µs

❯ poetry run python src/bin/day04-2-pyo3.py
Reading data from: data/day04.txt
Answer: 5489600
Time: 327.56 μs
```

## Day 05
 - First thing I tried was making a dictionary of all possible src -> dst mappings which worked fine with sample data but OOM'ed on "prod" data
 - Developed a nice Range -> Mapping -> Pipeline structure I liked, worked well for part 1 in Python and Rust
 - Part 2 is hard. Haven't figured out how to do it in Python
 - It feels like "collapsing" the transform ranges is doable but I am grug-brained. Brute force in Rust for the win
 - First time using Rayon! 

```
❯ poetry run python src/bin/day05-1.py
Reading data from: data/day05.txt
Answer: 662197086
Time: 358.17 μs

❯ cargo run --bin day05-1 --release -q
Reading data from: "./data/day05.txt"
Answer: 662197086
Time: 46.487µs

❯ poetry run python src/bin/day05-1-pyo3.py
Reading data from: data/day05.txt
Answer: 662197086
Time: 82.75 μs


❯ poetry run python src/bin/day05-2.py
Reading data from: data/day05.txt
Killed

❯ cargo run --bin day05-2 --release -q
Reading data from: "./data/day05.txt"
Input ranges: [1972667147..2378259165, ...]
Total number of items in flattened ranges: 2136279819
Answer: 52510809
Time: 29.662134129s

❯ poetry run python src/bin/day05-2-pyo3.py
Reading data from: data/day05.txt
Killed
```

# Day 06

 - Didn't try to optimize this much, just brute force it. Rust shines there

```
❯ poetry run python src/bin/day06-1.py
Reading data from: data/day06.txt
Answer: 781200
Time: 48.06 μs

❯ cargo run --bin day06-1 --release -q
Reading data from: "./data/day06.txt"
Answer: 781200
Time: 32.948µs

❯ poetry run python src/bin/day06-1-pyo3.py
Reading data from: data/day06.txt
Answer: 781200
Time: 39.00 μs


❯ poetry run python src/bin/day06-2.py
Reading data from: data/day06.txt
Answer: 49240091
Time: 3.24 s

❯ cargo run --bin day06-2 --release -q
Reading data from: "./data/day06.txt"
Answer: 49240091
Time: 70.204397ms

❯ poetry run python src/bin/day06-2-pyo3.py
Reading data from: data/day06.txt
Answer: 49240091
Time: 68.95 ms
```

# Day 08

 - learned that Rust has built in `itertools.cycle()` in `.chars().cycle()`