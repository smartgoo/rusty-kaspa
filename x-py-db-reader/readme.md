# Overview
An **experimental** Python package for reading rusty-kaspa's RocksDB instances. 

Uses PyO3 and Maturin.

# Setup
1. `cd ./rusty-kaspa/x-py-db-reader`
2. `python -m venv env`
3. `source env/bin/activate`
4. `pip install maturin`
5. `maturin develop --release`

# How to use
See `./python/test.py` for reference.

From the same python venv:
1. `from kaspadbr import Reader`
2. Refer to functions in `./src/core/reader.rs`

# TODO
WIP/up next:
- [ ] export_utxo_set() -> clean up TODOs throughout code
- [ ] get_utxo_set() -> returns utxo set in memory?
- [ ] update get_block() to retrieve header only or all block details
- [ ] get_unique_addresses() w/ params for balances=true/false, utxo_count=true/false
- [ ] write rust tests and python tests

Future:
- methods to retrieve data from all stores
- documentation
- get function argument suggestions/hints working in vscode, other editors 
- publish to pypi