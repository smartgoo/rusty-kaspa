# Overview
An **experimental** Python package for reading rusty-kaspa's RocksDB instances. 

Uses PyO3 and Maturin.

# Setup
1. Clone github project
2. `cd ./rusty-kaspa/x-kdbr`
3. `python -m venv env`
4. `source env/bin/activate`
5. `pip install maturin`
6. `maturin develop`

# How to use
See `./python/test.py` for reference.

From the same python venv:
1. `from kdbr import Reader`
2. Refer to functions in `./src/reader.rs`