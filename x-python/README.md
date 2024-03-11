# Overview
A project to build a Python package, natively from rusty-kaspa, for various aspects of Kaspa development in Python.

The following is under active **research and experimental development**:
- An async gRPC Client for rusy-kaspa
- Read rusty-kaspa's RocksDB instances

The following is slated for future **research and experimental development**:
- Wallet (address, transaciton, etc.) management

Since this is a fork of rusty-kaspa, all code for this project is inside of the `x-python` crate. There are very limited modifications to code in other crates, mainly for module visibility.


## Experimental!!!! 
This repository is for **research and experimentation**. The goal is to research how a Python package could be built natively from rusty-kaspa, similar to rusty-kaspa's [WASM-SDK](https://kaspa-mdbook.aspectron.com).


## Leveraging rusty-kaspa Source
Much of the functionality Python developers need already exits in the rusty-kaspa codebase. There's also some functionality that just can't be replicated in another language (to my knowledge, e.g. data serialization with Serde). So... why not try to leverage this existing Rust code directly in Python? 

That's the goal here! Rather than re-write functionality in Python, this project uses [PyO3](https://pyo3.rs/v0.20.0/) and [Maturin](https://www.maturin.rs) to build Rust bindings for Python. Resulting in a Python package that exposes official rusty-kaspa source for use in Python programs.

A great comparison is the WASM-SDK. The WASM-SDK is built directly from rusty-kaspa, providing bindings to Rust for JS developers.

---
# Getting Started

## Development Process:
The process to develop locally is:

1. Clone this project
2. Develop as desired. All code for this project is currently located in the `x-python` crate.
3. Use Maturin Develop to build and install the Python package in the active local virtual env:
    - `cd ./rusty-kaspa/x-python`
    - `source env/bin/activate` (create venv first if necessary: `python -m venv env`, activate, and `pip install maturin`)
    - `maturin develop --release` to build and install in the active Python venv.
4. Test the built Python package from within the same venv. Refer to code in `./x-python/python/tests/`.

## Build Wheel & Install in Another Python venv
To build a wheel for installation:

1. `cd ./rusty-kaspa/x-python`
2. `python -m venv env`
3. `source env/bin/activate`
4. `pip install maturin`
5. `maturin build --release` to build wheel
6. Copy the wheel filepath output by the command above: `📦 Built wheel for CPython <version> to <filepath>`.
7. Switch to the desired local venv. `pip install <filepath>`

## How to use from Python
See `./x-python/python/tests/` and `./x-python/kaspapy.pyi` (which is most likely out of date...) for reference.

Package name: `kaspapy`

Modules inside of `kaspapy` are `DBReader` and `grpc_client`. E.g:

- `from kaspapy import DBReader`
- `from kaspapy.grpc_client import AioClient`

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