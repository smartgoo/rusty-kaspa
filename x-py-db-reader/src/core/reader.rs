use crate::converters::ToPyDict;
use crate::core::{dirs::Dirs, stores::Stores};

use kaspa_consensus::model::stores::headers::HeaderStoreReader;
use kaspa_hashes::Hash;
use pyo3::prelude::*;
use std::{collections::HashSet, path::PathBuf, str::FromStr};

#[pyclass]
pub struct Reader {
    // Stores
    // To avoid Rust -> Python type conversion fun, not exposing this to Python
    stores: Stores,

    #[pyo3(get, set)]
    home_dir: PathBuf,

    #[pyo3(get, set)]
    app_dir: PathBuf,

    #[pyo3(get, set)]
    network_dir: PathBuf,

    #[pyo3(get, set)]
    db_dir: PathBuf,

    #[pyo3(get, set)]
    utxo_index_db_dir: Option<PathBuf>,

    #[pyo3(get, set)]
    meta_db_dir: PathBuf,

    #[pyo3(get, set)]
    consensus_db_dir: PathBuf,
}

#[pymethods]
impl Reader {
    #[new]
    pub fn new(app_dir: Option<PathBuf>, network: Option<String>) -> Self {
        // Init directories
        // TODO ensure app_dir exists
        let dirs = Dirs::new(app_dir.clone(), network.clone());

        // Init stores
        let stores = Stores::new(&dirs);

        Reader {
            stores,

            home_dir: dirs.home_dir,
            app_dir: dirs.app_dir,
            network_dir: dirs.network_dir,
            db_dir: dirs.db_dir,
            utxo_index_db_dir: dirs.utxo_index_db_dir,
            meta_db_dir: dirs.meta_db_dir,
            consensus_db_dir: dirs.consensus_db_dir,
        }
    }

    // Gets current consensus entry from meta store
    pub fn get_current_consensus_entry(&self) -> Option<u64> {
        self.stores.meta_store.get_current_consensus_entry()
    }

    // Gets block header for given hash from consensus DB. Returns a dict
    pub fn get_block_header(&self, py: Python, block_hash: String) -> PyResult<PyObject> {
        // TODO can I explicitly define PyDict as the return type?
        // Convert block_hash from String to Hash
        let block_hash = Hash::from_str(&block_hash).unwrap();

        // Get block from store
        let header = self.stores.headers_store.get_header(block_hash).unwrap();

        // Convert the header to a PyDict
        let header_dict = header.to_py_dict(py);

        // Convert the PyDict to PyObject and return it
        Ok(header_dict.to_object(py))
    }

    // Gets circulating supply from utxoindex store
    pub fn get_circulating_supply(&self) -> u64 {
        // TODO ensure utxoindex dir exists first
        self.stores.circulating_supply_store.as_ref().unwrap().get().unwrap()
    }

    // Gets utxo tips from utxoindex store
    pub fn get_utxo_tips(&self) -> HashSet<String> {
        // TODO use a converter and return PyResult<PyObject> or a list type if possible
        // TODO ensure utxoindex dir exists first

        // Get tips from store
        let utxo_tips = self.stores.utxo_tips_store.as_ref().unwrap().get().unwrap();

        // Return as HashSet<String> (rather than BlockHashSet) for ease of type conversion w/ PyO3
        let mut tips = HashSet::new();
        for tip in utxo_tips.iter() {
            tips.insert(tip.to_string());
        }

        tips
    }

    // Exports entire UTXO set to a CSV file. Returns count of UTXOs exported.
    #[pyo3(signature = (
        filepath,
        address=true,
        daa_score=true,
        amount=true,
        is_coinbase=true,
        outpoint=false,
        chunk_size=100000,
        verbose=false
    ))]
    pub fn export_utxo_set(
        &self,
        filepath: String, // TODO PyO3 was throwing some error on &str
        address: bool,
        daa_score: bool,
        amount: bool,
        is_coinbase: bool,
        outpoint: bool,
        chunk_size: i32,
        verbose: bool,
    ) -> i64 {
        self.stores.utxo_store.as_ref().unwrap().export_all_outpoints(
            filepath,
            address,
            daa_score,
            amount,
            is_coinbase,
            outpoint,
            chunk_size,
            verbose,
        )
    }
}
