use crate::core::dirs::Dirs;
use crate::stores::indexed_utxos::DbUtxoSetByScriptPublicKeyStore;

use pyo3::prelude::*;
use std::path::PathBuf;

#[pyclass]
#[pyo3(name = "UtxoIndexStore")]
pub struct PyUtxoIndexStore {
    // To avoid Rust -> Python type conversion fun, not exposing this to Python
    inner_store: DbUtxoSetByScriptPublicKeyStore,
}

#[pymethods]
impl PyUtxoIndexStore {
    #[new]
    pub fn new(py: Python, app_dir: Option<PathBuf>, network: Option<String>) -> PyResult<Self> {
        // Init dirs
        let dirs = Dirs::new(app_dir.clone(), network.clone());

        // Check that app dir exists
        if !&dirs.validate_existence() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("issue with rusty-kaspa directories."));
        }

        // Check that utxo index dir exists
        // TODO

        // Create utxo index db connection
        let utxo_index_db = kaspa_database::prelude::ConnBuilder
            ::default()
            .with_db_path(dirs.utxo_index_db_dir.clone().unwrap())
            .with_files_limit(10) // TODO
            .build()
            .unwrap();

        // Init inner store
        let inner_store = DbUtxoSetByScriptPublicKeyStore::new(utxo_index_db.clone(), 0);

        Ok(PyUtxoIndexStore {
            inner_store
        })
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
    pub fn export(
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
        Ok(self.inner_store.export_all_outpoints(
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