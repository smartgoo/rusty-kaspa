use crate::core::dirs::Dirs;

use kaspa_utxoindex::stores::{
    supply::{CirculatingSupplyStoreReader, DbCirculatingSupplyStore}, 
};
use pyo3::prelude::*;
use std::path::PathBuf;

#[pyclass]
#[pyo3(name = "CirculatingSupplyStore")]
pub struct PyCirculatingSupplyStore {
    // To avoid Rust -> Python type conversion fun, not exposing this to Python
    inner_store: DbCirculatingSupplyStore,

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
impl PyCirculatingSupplyStore {
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
        let inner_store = DbCirculatingSupplyStore::new(utxo_index_db.clone());

        Ok(PyCirculatingSupplyStore {
            inner_store,
            home_dir: dirs.home_dir,
            app_dir: dirs.app_dir,
            network_dir: dirs.network_dir,
            db_dir: dirs.db_dir,
            utxo_index_db_dir: dirs.utxo_index_db_dir,
        })
    }

    pub fn get(&self) -> PyResult<u64> {
        Ok(self.inner_store.get().unwrap())
    }
}