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
### WIP:
- Exporting UTXO set:
    - [x] Rename Reader fn `export_all_utxos()` to `export_utxo_set()`
    - [ ] Handle `script_public_key` param properly in `DbUtxoSetByScriptPublicKeyStore.export_all_outpoints()`
    - [ ] Add `outpoint` param (true/false, default=false?). If true, includes outpoint tx and index
    - [ ] Optmized for `for r in chunk` loop. Should only parse `key` based passed params
    - [x] Decide on what should be returned. Currently is count of utxos exported.
- [ ] clean up TODOs throughout code


### Next:
- Undo fully custom store implementations in `x-py-db-reader/src/stores` and use oob stores (with some minor visibility modifications and extended functionality where required).
- Write `export_unique_addresses()` fn. Support params for `balances`=true/false, `utxo_count`=true/false
- Write `get_unique_addresses()` fn. Support params for `balances`=true/false, `utxo_count`=true/false
- Switch Reader `get_block_header()` to `get_block()` with `header_only` param. Update fn to include all block details accordingly
- Write rust tests and python tests

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