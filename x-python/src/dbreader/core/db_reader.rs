use crate::dbreader::core::dirs::Dirs;
use crate::dbreader::core::stores::Stores;

use pyo3::prelude::*;
use std::path::PathBuf;

#[pyclass]
pub struct DBReader {
    // DIRECTORIES
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

    // STORES
    #[pyo3(get)]
    stores: Stores,
}

#[pymethods]
impl DBReader {
    #[new]
    pub fn new(_py: Python, app_dir: Option<PathBuf>, network: Option<String>) -> PyResult<Self> {
        // Init directories
        let dirs = Dirs::new(app_dir.clone(), network.clone());

        // Check that dirs exists and throw error if not
        if !&dirs.validate_existence() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("issue with rusty-kaspa directories."));
        }

        // Init wrapped stores (with inner store having the DB conn)
        let stores = Stores::new(&dirs).unwrap();

        Ok(Self {
            home_dir: dirs.home_dir,
            app_dir: dirs.app_dir,
            network_dir: dirs.network_dir,
            db_dir: dirs.db_dir,
            utxo_index_db_dir: dirs.utxo_index_db_dir,
            meta_db_dir: dirs.meta_db_dir,
            consensus_db_dir: dirs.consensus_db_dir,

            stores,
        })
    }
}