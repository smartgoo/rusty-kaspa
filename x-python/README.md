# Overview
An experimental Python package for reading from Rusty Kaspa Rocks DB. Built from rust source.


# Building from Rust
This project uses [PyO3](https://pyo3.rs/v0.20.0/) and [Maturin](https://www.maturin.rs) to build Rust bindings for Python. The result is a Python package that exposes rusty-kaspa/rust source for use in Python programs.

---

# Getting Started

## Project Layout
The Python package `kaspadbr` is built from the `x-python` crate (with dependencies on other creates in the workspace). 

A good starting point the `kaspadbr` struct in `x-python/src/lib.rs`, and the `DBReader` struct in `x-python/src/core/db_reader.rs`.


## Development Process:
To develop locally:

1. Get a copy of this code
2. Develop as desired
3. `cd ./x-python`
4. Create/activate python virtual environment, install maturin (`pip install maturin`)
5. Use `maturin develop` to build and install the Python package in the active virtual env: `maturin develop --release`
6. Test the built Python package from within the same venv. Refer to code in `./x-python/python/usage/`

## Build Wheel & Install in Another Python Env
To build a wheel for installation:

1. `cd ./x-python`
2. Create/activate python virtual environment, install maturin (`pip install maturin`)
3. Use `maturin build` to build wheel: `maturin build --release`
4. Copy the wheel filepath output by the command above: `📦 Built wheel for CPython <version> to <filepath>`.
5. Switch to the desired local venv. `pip install <filepath>`

## How to use from Python
Package name: `kaspadbr`

See `./x-python/python/usage/`