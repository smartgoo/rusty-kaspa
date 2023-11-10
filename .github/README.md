# Overview
An **experimental** Python package for reading rusty-kaspa's RocksDB instances. 

Built on top of [rusty-kaspa](https://github.com/kaspanet/rusty-kaspa), using [PyO3](https://pyo3.rs/v0.20.0/) and [Maturin](https://www.maturin.rs) to create Rust bindings for Python. This project is a fork of [rusty-kaspa](https://github.com/kaspanet/rusty-kaspa) with slight (very limited) modifications to source, and an additional library crate. 

The additional crate `x-kaspadbr` is where all code specific to this project can be found.

# Development Process:
**First**, develop rust code. All code for this project is in `x-kaspadbr` crate. Although code from other rusty-kaspa crates can be used as well.

**Second**, build and install the Python package in local virtual env:
1. `cd ./rusty-kaspa/x-kaspadbr`
2. `python -m venv env`
3. `source env/bin/activate`
4. `pip install maturin`
5. `maturin develop --release` to build and install in the active Python venv.

**Third**, test the built Python package. Refer to code in `./x-kaspadbr/python/`.

# How to use from Python
See `./x-kaspadbr/python/` and `./x-kaspadbr/kaspadbr.pyi`.

# Roadmap
Misc. features that might be added to this package over time:

Two primary APIs at different abstraction levels:
- A more abstracted API similar to RPC methods
- 1:1 with OOB rusty-kaspa stores

Utilities to dump/export data.
Utilities to iterate stores

# TODO
A lot. But these are next steps:
- [ ] Find the right project structure that allows modular, organized Rust code while providing a Pythonic module interface.
- Undo custom indexed_utxos.rs store. Use OOB indexed_utxos.rs and extend functionality
- Undo custom meta.rs store. Use OOB meta.rs with and extended functionality
- Iterate UTXO set in chunks using PyO3 iterator

Then:
- Slowly add functions to read all stores and data types