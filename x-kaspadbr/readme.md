# Overview
An **experimental** Python package for reading rusty-kaspa's RocksDB instances. 

Written in rust, bindings generated with PyO3 and Maturin.

# Install
Package is not yet available on PyPi. Here is how you would install in your local environment:
1. clone the project
2. `cd ./rusty-kaspa/x-kaspadbr`
3. `python -m venv env`
4. `source env/bin/activate`
5. `pip install maturin`
6. `maturin build --release`
7. Copy wheel filepath output by command above: `Built wheel for CPython <version> to <filepath>`
7. Install the built wheel in your desired env: `pip install <filepath>`

# How to use
See `./x-kaspadbr/python/test.py` for reference.

Refer to `./x-kaspadbr/kaspadbr.pri` and `./x-kaspadbr/src/core/reader.rs` for more information.

# TODO
### WIP:
- `export_utxo_set()`:
    - [ ] Tests for `export_utxo_set()` stack
    - [ ] Thoroughly document entire `export_utxo_set()` stack in rust and pyo3 compliant manner
- [ ] Undo custom store `x-py-db-reader/src/stores/utxoindex/indexed_utxos.rs` and use oob store.
- [ ] Undo custom store `x-py-db-reader/src/stores/utxoindex/meta.rs` and use oob store.
- [ ] Add fn `utxo_set_iterator()` to iterate over utxo set in chunks from Python


### Next:
- Properly handle database/src/errors.rs StoreError in all fns
- Should `export_utxo_set()` expose param that dictates whether or not to export ScriptPublicKey?
- Write `export_unique_addresses()` fn. Support params for `balances`=true/false, `utxo_count`=true/false
- Write `get_unique_addresses()` fn. Support params for `balances`=true/false, `utxo_count`=true/false
- Switch Reader `get_block_header()` to `get_block()` with `header_only` param. Update fn to include all block details accordingly
- Comprehensive rust tests and python tests

### Future:
- Add `daa_timestamp` param (true/false, default=false) to `Reader.export_utxo_set()`. If true, uses rusty-kaspa oob daa estimation fn. Waiting for [coderofstuff's DAA estimation PR](https://github.com/kaspanet/rusty-kaspa/pull/268) to be merged into rusty-kaspa master
- SQL style `where` filtering for UTXO set, all other bulk data types
- Methods to retrieve data from all stores
- Documentation
- Get function argument suggestions/hints working in vscode, other editors 
- Publish to pypi

### Considering:
- Get entire UTXO set in memory
- Get all unique addresses in memory