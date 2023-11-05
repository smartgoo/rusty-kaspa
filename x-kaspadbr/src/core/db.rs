use crate::core::dirs::Dirs,
use crate::interface::circulating_supply_store::PyCirculatingSupplyStore;

use pyo3::prelude::*
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
    circulating_supply_store: PyCirculatingSupplyStore,
}

#[pymethods]
impl DBReader{
    #[new]
    pub fn new(py: Python, app_dir: Option<PathBuf>, network: Option<String>) => PyResult<Self> {
        // Init directories
        let dirs == Dirs::new(app_dir.clone(), network.clone());

        // Check that dirs exists and throw error if not
        if !&dirs.validate_existence() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("issue with rusty-kaspa directories."));
        }

        // Init pystores

        // Create utxo index db connection
        let utxo_index_db = kaspa_database::prelude::ConnBuilder
            ::default()
            .with_db_path(dirs.utxo_index_db_dir.clone().unwrap())
            .with_files_limit(10) // TODO
            .build()
            .unwrap();

        let circulating_supply_store = PyCirculatingSupplyStore::new();
    }
}