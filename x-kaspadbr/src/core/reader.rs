use crate::converters::{ToPyDict, transaction::transactions_to_py_list};
use crate::core::{dirs::Dirs, stores::Stores};

use kaspa_consensus::model::stores::headers::HeaderStoreReader;
use kaspa_consensus::model::stores::block_transactions::BlockTransactionsStoreReader;
use kaspa_database::prelude::StoreError;
use kaspa_hashes::Hash;
use pyo3::prelude::*;
use pyo3::types::{PyDict};
use std::{path::PathBuf, str::FromStr};

#[pyclass]
pub struct Reader {
    // Stores
    // To avoid Rust -> Python type conversion fun, not exposing this to Python
    stores: Stores,

    #[pyo3(get)]
    home_dir: PathBuf,

    #[pyo3(get)]
    app_dir: PathBuf,

    #[pyo3(get)]
    network_dir: PathBuf,

    #[pyo3(get)]
    db_dir: PathBuf,

    #[pyo3(get)]
    utxo_index_db_dir: Option<PathBuf>,

    #[pyo3(get)]
    meta_db_dir: PathBuf,

    #[pyo3(get)]
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
    pub fn get_block_header(&self, py: Python, block_hash: String, include_transactions: bool) -> PyResult<PyObject> {
        // TODO checkout consensus/src/consensus/mod.rs:636
        
        // Convert block_hash from String to Hash
        let block_hash = Hash::from_str(&block_hash).unwrap();

        // Get header
        let header = match self.stores.headers_store.get_header(block_hash) {
            Ok(header) => Some(header),
            Err(StoreError::KeyNotFound(_)) => None,
            _ => todo!(), // TODO rest of potential StoreErrors
        };

        // If header is None, block isn't found. Return
        if header.is_none() {
            return Ok(py.None());
        }

        // If transactions param is true, attempt to get transactions
        let transactions = if include_transactions {
            match self.stores.block_transactions_store.get(block_hash) {
                Ok(transactions) => Some(transactions),
                Err(StoreError::KeyNotFound(_)) => None,
                _ => todo!(), // TODO rest of potential StoreErrors
            }
        } else {
            None
        };

        // Create a binding to the unwrapped header to extend its lifetime
        let unwrapped_header = header.unwrap();
        let header_dict = unwrapped_header.to_py_dict(py);

        // Build return dict
        let dict = PyDict::new(py);
        dict.set_item("header", header_dict).unwrap();

        // Convert transactions to Python dict
        if include_transactions {
            let transaction_list = transactions_to_py_list(py, (&transactions.unwrap()).to_vec());
            dict.set_item("transactions", transaction_list).unwrap();
        }

        Ok(dict.to_object(py))
    }
}
