# aoc_2023

Advent of Code 2023 - Rust + Python

# Project Layout

```
/notebooks
  day01-1-py.ipynb <-- full training and prod Python solution here
  day01-1-rs.ipynb <-- prototype Rust here, no full solution
  day01-2-py.ipynb
  day01-2-rs.ipynb
  ...

/prod
  .gitkeep
  ... <- pasting my own prod data files here, gitignored

/src
  lib.rs
  utils.rs <-- helper functions for reading files from train/prod folder
  bin/
    day01-1.rs <-- full Rust solution for train and prod set

/train
  day01-1.txt <-- sample/training data for day01 problem 1
  day02-2.txt
  ...

Cargo.toml <-- Rust deps
pyproject.toml <-- Python deps
```

# Notebooks

`poetry run jupyter notebook` should start Jupyter and use ipykernel from the Poetry-managed `.venv`, with any dependencies in that virtual-env installed. You can check by running `sys.executable` in the Notebook.

Choosing a Rust kernel will use the system-wide install of [evcxr](https://crates.io/crates/evcxr_jupyter), see documentation there for installation. To use third-party crates on the Rust kernel, include `:deps` in a cell.

# Rust binaries

Rust solutions will be run as binaries. For day01 problem 1, the command will look like:

- `cargo run --bin day01-1` <-- read from `train/` dataset and execute
- `PROD=1 cargo run --bin day01-1` <-- read from `prod/` dataset and execute

Optionally, `--release` can be added to build and run in release mode.