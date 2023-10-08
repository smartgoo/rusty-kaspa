# Overview
An **experimental** project to create a Python package for reading rusty-kaspa's RocksDB instances. 

Uses PyO3 and Maturin.

# Setup
1. Clone github project
2. `cd ./rusty-kaspa/x-py03`
3. `python -m venv env`
4. `source env/bin/activate`
5. `pip install maturin`
6. `maturin develop`

# How to use
From the same python venv:
1. `import rusty_kaspa_db`