# Overview
An experimental project to build a (comprehensive) Kaspa Python SDK natively from rusty-kaspa.

The main Python module lives inside the `x-python` crate. There are additional files modified or added throughout other crates.


## Experimental!!!! 
This repository is for **research and experimentation**. The goal is to research how a Python package could be built natively from rusty-kaspa, similar to Kaspa's [WASM-SDK](https://kaspa-mdbook.aspectron.com).


## Leveraging rusty-kaspa Source
Much of the functionality Python developers need already exits in the rusty-kaspa codebase. There's also functionality that just can't be replicated in another language (to my knowledge, e.g. data serialization with Serde). So... why not try to leverage this existing Rust code directly in Python? 

Rather than rewrite functionality in Python, this project uses [PyO3](https://pyo3.rs/v0.20.0/) and [Maturin](https://www.maturin.rs) to build Rust bindings for Python. Resulting in a Python package that exposes rusty-kaspa source for use in Python programs.

---

# Getting Started

## Project Layout
This is a fork of rusty-kaspa. The Python package `kaspapy` is built from the `x-python` crate. As such, the `kaspapy` struct in `x-python/src/lib.rs` is a good starting point. This struct uses PyO3 to add functionality (structs and functions throughout the various workspace crates). The Maturin build tool then builds a Python module from the `kaspapy` struct, including all functionality.


## Development Process:
The process to develop locally is:

1. Clone this project
2. Develop as desired
3. Use Maturin `develop` to build and install the Python package in the active local virtual env:
    - `cd ./x-python`
    - `source env/bin/activate` (create venv first if necessary: `python -m venv env`, activate, and `pip install maturin`)
    - `maturin develop --release` to build and install in the active Python venv
4. Test the built Python package from within the same venv. Refer to code in `./x-python/python/uage/`

## Build Wheel & Install in Another Python venv
To build a wheel for installation:

1. `cd ./x-python`
2. Use Maturin `build` to build the Python package:
    - `cd ./x-python`
    - `source env/bin/activate` (create venv first if necessary: `python -m venv env`, activate, and `pip install maturin`)
    - `maturin build --release` to build
3. Copy the wheel filepath output by the command above: `📦 Built wheel for CPython <version> to <filepath>`.
4. Switch to the desired local venv. `pip install <filepath>`

## How to use from Python
Package name: `kaspapy`

See `./x-python/python/usage/` and `./x-python/kaspapy.pyi` (which is definitely out of date...) for reference.

---

# TODO
- [ ] Update `kaspapy.pyi`
- [ ] Get `pyproject.toml` setup right
- [ ] Get `Cargo.toml` setup right
- [ ] Find right structure for `x-python` crate
- [ ] Standardize package structure/naming convention

`DBReader`:
- [ ] Add other RocksDB stores to `DBReader` that are currently not exposed via RPC

`gRPC Client`:
- [ ] AIO gRPC client
- [ ] Sync gRPC client
- [ ] wRPC client?
- [ ] Return responses in Pydantic/dataclasses ?