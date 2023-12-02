# aoc_2023

Advent of Code 2023 - Rust + Python

# Project Layout

```
/data
  day01.txt <-- "prod" problem data for @kafonek AoC account
  day02.txt
  ...

/notebooks
  day01-1-py.ipynb <-- prototype and solve "prod" problem with %%timeit
  day01-1-rs.ipynb <-- prototype Rust here, no full solution
  day01-2-py.ipynb
  day01-2-rs.ipynb
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

Cargo.toml <-- Rust deps
pyproject.toml <-- Python deps
```

# Notebooks

`poetry run jupyter notebook` should start Jupyter and use ipykernel from the Poetry-managed `.venv`, with any dependencies in that virtual-env installed. You can check by running `sys.executable` in the Notebook.

Choosing a Rust kernel will use the system-wide install of [evcxr](https://crates.io/crates/evcxr_jupyter), see documentation there for installation. To use third-party crates on the Rust kernel, include `:deps` in a cell. Evcxr will cache the download but not the compilation step by default, so using `:deps` is a bit slow. There is an option for [caching compilation in the evcxr documentation](https://github.com/evcxr/evcxr/blob/main/COMMON.md#caching) but it didn't seem to work for me and there are [various](https://github.com/evcxr/evcxr/issues/218) [github](https://github.com/evcxr/evcxr/issues/304) [issues](https://github.com/evcxr/evcxr/issues/319) on the topic. 

# Rust binaries

Rust solutions will be run as binaries. They should be run with release mode, e.g. `cargo run --bin day01-1 --release`.

# Problem Notes

## Day 01
 - Part 2 was quite gimmicky imo with its wrinkle that spelled-out numbers could overlap in letters, like "eightwo" would match 8 and 2. Creating a regex pattern like "one|two|three|..." and doing normal match-iteration didn't work because regex by default consumes the longest match. To get around that, used a hacky "find a match then start looking again from match start index + 1" approach.

## Day 02
 - Much easier than the day 01 part 2
 - Really liked the symmetry between the Python and Rust code in this problem