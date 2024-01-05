# AOC 2023

My goal at the outset of [Advent of Code 2023](https://adventofcode.com/2023) was to figure out good workflows for me to prototype solutions in Python, then convert them to Rust and measure performance benchmarks. I had one stretch goal of learning how to incorporate `pyo3` and `maturin` to create Python bindings to the pure Rust solutions. A second stretch goal was to learn how to use the Rust solutions in WebAssembly.

After several iterations on directory structure and layout, I think I have found a process I like.

1. Prototype in Python
   1. Begin solving the "demo" problem for each Day/Part in a Jupyter Notebook (`./notebooks/python/01-1.ipynb`)
   2. Move functions and classes used between parts into a flat file (`./python/aoc_2023/day01.py`)
   3. Create a flat file for printing out answer and timing (`./python/bin/01-1.py`)  
   4. Run code (`poetry run python bin/01-1.py`)

2. Convert to Rust
   1. Rewrite Python functions/classes in Rust (`./rust/src/day01.rs`)
   2. Create answer and timing script (`./rust/src/bin/01-1.rs`)
   3. Run code (`cargo run --bin 01-1 --release`)

3. Python bindings to Rust
   1. Create `pyo3` bindings to pure Rust code (`./rust-pyo3/src/day01.rs`)
   2. Create Python stub files for editor support (`./rust-pyo3/aoc_2023_pyo3/day01.pyi`)
   3. Copy/paste pure Python answer and timing code, only changing the import from `aoc_2023` to `aoc_2023_pyo3` (`./rust-pyo3/bin/01-1.py`)
   4. Create Python wheel (`poetry run maturin develop --release`)
   5. Run code (`poetry run python bin/01-1.py`)

4. WASM bindings to Rust
   1. Pull in `aoc_2023` crate and copy/paste solution logic into a function (`./rust-wasm/src/day01.rs`)
   2. Add the day/part solution functions to a single `solve` function wrapped with `#[wasm-bindgen]` (`./rust-wasm/src/lib.rs`)
   3. Run `wasm-pack build --target web --release` to create `./rust-wasm/pkg` directory
   4. Update `./rust-wasm/index.html` to add the appropriate dropdowns / radio buttons, that file contains the javascript entrypoint to call the wasm functions
   5. Test by serving out files `python -m http.server`
   6. Commit and push to Github, let CI build [kafonek.github.io/aoc_2023](https://kafonek.github.io/aoc_2023/) page



# Timings

The below times are recorded from running on my desktop, and not averaged over multiple executions. Treat them with a grain of salt.

 - Python numbers: `cd python && poetry run python bin/<day-part>.py`
 - Rust numbers: `cd rust && cargo run --bin <day-part> --release`
 - Pyo3 numbers: `cd rust-pyo3 && maturin develop --release && poetry run python bin/<day-part>.py`

| Day | Part | Python        | Rust              | Pyo3               |
|-----|------|---------------|-------------------|--------------------|
| 1   | 1    | 850 µs        | 150 µs            | 335 µs             |
| 1   | 2    | 3.9 ms        | 650 µs            | 870 µs             |
| 2   | 1    | 890 µs        | 100 µs            | 170 µs             |
| 2   | 2    | 1.05 ms       | 100 µs            | 165 µs             |
| 3   | 1    | 14 ms         | NA (gridthings-rs WIP) | NA (gridthings-rs WIP) |
| 3   | 2    | 14 ms         | NA (gridthings-rs WIP) | NA (gridthings-rs WIP) |
| 4   | 1    | 1.05 ms       | 150 µs            | 220 µs             |
| 4   | 2    | 1.15 ms       | 165 µs            | 330 µs             |
| 5   | 1    | 358 µs        | 45 µs             | 90 µs              |
| 5   | 2    | NA (timeout)  | 28s               | NA (timeout)       |
| 6   | 1    | 55 µs         | 6 µs              | 50 µs              |
| 6   | 2    | 4.17 s        | 74 ms             | 74 ms              |
| 7   | 1    | 7.5 ms        | 375 µs            | 900 µs             |
| 7   | 2    | 7.5 ms        | 420 µs            | 930 µs             |


