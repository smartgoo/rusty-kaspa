use crate::core::dirs::Dirs;

use kaspa_utxoindex::stores::tips::{DbUtxoIndexTipsStore, UtxoIndexTipsStoreReader};
use pyo3::prelude::*;
use std::path::PathBuf;

#[pyclass]
#[pyo3(name = "UtxoIndexTipsStore")]
pub struct PyUtxoIndexTipsStore {
    inner_store: DbUtxoIndexTipsStore,

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
}

#[pymethods]
impl PyUtxoIndexTipsStore {
    #[new] 
    pub fn new(py: Python, app_dir: Option<PathBuf>, network: Option<String>) -> PyResult<Self> {
        // Init directories
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
        let inner_store = DbUtxoIndexTipsStore::new(utxo_index_db.clone());

        Ok(PyUtxoIndexTipsStore {
            inner_store,
            home_dir: dirs.home_dir,
            app_dir: dirs.app_dir,
            network_dir: dirs.network_dir,
            db_dir: dirs.db_dir,
            utxo_index_db_dir: dirs.utxo_index_db_dir,
        })
    }   

    pub fn get(&self) -> PyResult<Vec<String>> {
        // Get tips from store
        let utxo_tips = self.inner_store.get().unwrap();

        // Return as Vec<String> (rather than BlockHashSet) for ease of type conversion w/ PyO3
        let mut tips = Vec::new();
        for tip in utxo_tips.iter() {
            tips.push(tip.to_string());
        }

        Ok(tips)
    }
}