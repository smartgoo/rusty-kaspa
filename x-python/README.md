# Overview
An experimental project to build a (comprehensive) Kaspa Python SDK natively from rusty-kaspa.

The Python module is built from rust source code that lives in the `x-python` crate. There are additional files modified or added throughout other crates.


# Building from Rust
This project uses [PyO3](https://pyo3.rs/v0.20.0/) and [Maturin](https://www.maturin.rs) to build Rust bindings for Python. The result is a Python package that exposes rusty-kaspa/rust source for use in Python programs.

---

# Getting Started

## Project Layout
The Python package `kaspapy` is built from the `x-python` crate. As such, the `kaspapy` struct in `x-python/src/lib.rs` is a good starting point. This struct uses PyO3 to add functionality (structs and functions throughout the various rusty-kaspa crates). The Maturin build tool then builds a Python module from the `kaspapy` struct, including all functionality.


## Development Process:
The process to develop locally is:

1. Get a copy of this code
2. Develop as desired
3. Use `maturin develop` to build and install the Python package in the active local virtual env:
    - `cd ./x-python`
    - `source env/bin/activate` (create venv first if necessary and `pip install maturin`)
    - `maturin develop --release` to build and install in the active Python venv
4. Test the built Python package from within the same venv. Refer to code in `./x-python/python/usage/`

## Build Wheel & Install in Another Python venv
To build a wheel for installation:

1. `cd ./x-python`
2. Use `maturin build` to build the Python package:
    - `cd ./x-python`
    - `source env/bin/activate` (create venv first if necessary and `pip install maturin`)
    - `maturin build --release` to build
3. Copy the wheel filepath output by the command above: `📦 Built wheel for CPython <version> to <filepath>`.
4. Switch to the desired local venv. `pip install <filepath>`

## How to use from Python
Package name: `kaspapy`

See `./x-python/python/usage/`

---

# TODO
A lot of things, as well as the following
- [ ] Find right structure for `x-python` crate
- [ ] Update `kaspapy.pyi`
- [ ] Get `pyproject.toml` setup right
- [ ] Get `Cargo.toml` setup right
- [ ] gRPC client

`DBReader`:
- [ ] Merge DBReader into this
- [ ] Add other RocksDB stores to `DBReader` that are currently not exposed via RPC