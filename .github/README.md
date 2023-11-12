# Overview
An **experimental** Python package for reading rusty-kaspa's RocksDB instances. 

Built on top of [rusty-kaspa](https://github.com/kaspanet/rusty-kaspa), using [PyO3](https://pyo3.rs/v0.20.0/) and [Maturin](https://www.maturin.rs) to create Rust bindings for Python. This project is a fork of [rusty-kaspa](https://github.com/kaspanet/rusty-kaspa) with slight (very limited) modifications to source, and an additional library crate. 

The additional crate `x-kaspadbr` is where all code specific to this project can be found.

# Development Process:
Should you wish to develop this locally, here is the process.

**First**, clone this project.

**Second**, develop as needed. All code for this project is currently located in the `x-kaspadbr` crate.

**Third**, use Maturin Develop to build and install the Python package in local virtual env for testing purposes.
1. `cd ./rusty-kaspa/x-kaspadbr`
2. `source env/bin/activate` (create venv first if necessary: `python -m venv env`, activate, and `pip install maturin`)
5. `maturin develop --release` to build and install in the active Python venv.
7. Test the built Python package from within the same venv. Refer to code in `./x-kaspadbr/python/`.

# Install in a local Python venv
1. `cd ./rusty-kaspa/x-kaspadbr`
2. `python -m venv env`
3. `source env/bin/activate`
4. `pip install maturin`
5. `maturin build --release` to build wheel
6. Copy the wheel filepath output by the command above: `📦 Built wheel for CPython <version> to <filepath>`.
7. Switch to the desired local venv. `pip install <filepath>`

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