# Overview
An **experimental** Python package for reading rusty-kaspa's RocksDB instances. 

Built on top of [rusty-kaspa](https://github.com/kaspanet/rusty-kaspa), using [PyO3](https://pyo3.rs/v0.20.0/) and [Maturin](https://www.maturin.rs) to create Rust bindings for Python. This project is a fork of [rusty-kaspa](https://github.com/kaspanet/rusty-kaspa) with slight (very limited) modifications to source, and an additional library crate. 

The additional crate `x-kaspadbr` is where all code specific to this project can be found.

# Install in another Python venv
Package is not yet available on PyPi. Here is how to install in local Python venv:
1. clone the project
2. `cd ./rusty-kaspa/x-kaspadbr`
3. `python -m venv env`
4. `source env/bin/activate`
5. `pip install maturin`
6. `maturin build --release`
7. Command above with output a filepath to the built wheel: `Built wheel for CPython <version> to <filepath>`
8. Switch to your desired env and install from the wheel: `pip install <filepath>`

# Maturin Develop
1. clone the project
2. `cd ./rusty-kaspa/x-kaspadbr`
3. `python -m venv env`
4. `source env/bin/activate`
5. `pip install maturin`
6. `maturin develop --release` to build and install in the active Python venv.

# How to use
See `./x-kaspadbr/python/test.py` for reference.

Refer to `./x-kaspadbr/kaspadbr.pyi` and `./x-kaspadbr/src/core/reader.rs` for more information.

# TODO
A lot