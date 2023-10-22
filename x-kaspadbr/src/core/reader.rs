use crate::converters::ToPyDict;
use crate::core::{dirs::Dirs, stores::Stores};

use kaspa_consensus::model::stores::headers::HeaderStoreReader;
use kaspa_database::prelude::StoreError;
use kaspa_hashes::Hash;
use kaspa_utxoindex::stores::{supply::CirculatingSupplyStoreReader, tips::UtxoIndexTipsStoreReader};
use pyo3::prelude::*;
use std::{path::PathBuf, str::FromStr};

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
    pub fn new(py: Python, app_dir: Option<PathBuf>, network: Option<String>) -> PyResult<Self> {
        // Init directories
        let dirs = Dirs::new(app_dir.clone(), network.clone());

        // Check that dirs exists and throw error if not
        if !&dirs.validate_existence() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("issue with rusty-kaspa directories."));
        }

        // Init stores
        let stores = Stores::new(&dirs);

        Ok(Reader {
            stores,

            home_dir: dirs.home_dir,
            app_dir: dirs.app_dir,
            network_dir: dirs.network_dir,
            db_dir: dirs.db_dir,
            utxo_index_db_dir: dirs.utxo_index_db_dir,
            meta_db_dir: dirs.meta_db_dir,
            consensus_db_dir: dirs.consensus_db_dir,
        })
    }

    fn check_utxoindex_existence(&self) -> PyResult<()> {
        if self.utxo_index_db_dir.is_none() {
            return Err(PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>("utxoindex not found. Please run node with --utxoindex."));
        }
        Ok(())
    }

    // Gets current consensus entry from meta store
    pub fn get_current_consensus_entry(&self) -> Option<u64> {
        self.stores.meta_store.get_current_consensus_entry()
    }

    // Gets block header for given hash from consensus DB. Returns a dict
    pub fn get_block_header(&self, py: Python, block_hash: String) -> PyResult<PyObject> {
        // Convert block_hash from String to Hash
        let block_hash = Hash::from_str(&block_hash).unwrap();

        // Get block from store
        match self.stores.headers_store.get_header(block_hash) {
            Ok(header) => {
                // Convert the header to a PyDict
                let header_dict = header.to_py_dict(py);

                // Convert the PyDict to PyObject and return it
                Ok(header_dict.to_object(py))
            },
            Err(StoreError::KeyNotFound(_)) => Ok(py.None().to_object(py)), // Return None for KeyNotFound
            _ => todo!(), // TODO rest of potential StoreErrors
        }
    }

    // Gets circulating supply from utxoindex store
    pub fn get_circulating_supply(&self) -> PyResult<u64> {
        // Throw error if UTXO index doesn't exist
        self.check_utxoindex_existence()?;

        Ok(self.stores.circulating_supply_store.as_ref().unwrap().get().unwrap())
    }

    // Gets utxo tips from utxoindex store
    pub fn get_utxo_tips(&self) -> PyResult<Vec<String>> {
        // Throw error if UTXO index doesn't exist
        self.check_utxoindex_existence()?;

        // Get tips from store
        let utxo_tips = self.stores.utxo_tips_store.as_ref().unwrap().get().unwrap();

        // Return as Vec<String> (rather than BlockHashSet) for ease of type conversion w/ PyO3
        let mut tips = Vec::new();
        for tip in utxo_tips.iter() {
            tips.push(tip.to_string());
        }

        Ok(tips)
    }

    /// Exports entire UTXO set to a CSV file. Returns count of UTXOs exported.
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
        filepath: String,
        address: bool,
        daa_score: bool,
        amount: bool,
        is_coinbase: bool,
        outpoint: bool,
        chunk_size: i32,
        verbose: bool,
    ) -> PyResult<i64> {
        // Throw error if UTXO index doesn't exist
        self.check_utxoindex_existence()?;

        Ok(self.stores.utxo_store.as_ref().unwrap().export_all_outpoints(
            filepath,
            address,
            daa_score,
            amount,
            is_coinbase,
            outpoint,
            chunk_size,
            verbose,
        ))
    }
}
