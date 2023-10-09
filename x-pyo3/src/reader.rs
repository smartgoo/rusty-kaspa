use std::{
    collections::HashSet,
    path::PathBuf,
};
use pyo3::prelude::*;
use crate::core::{
    dir_manager::DirManager, 
    store_manager::StoreManager
};

#[pyclass]
pub struct Reader {
    // Stores
    // To avoid Rust -> Python type conversion fun, not exposing this to Python
    store_manager: StoreManager,

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
        let dirs = DirManager::new(app_dir.clone(), network.clone());

        // Init store manager
        let store_manager = StoreManager::new(&dirs);

        Reader {
            store_manager,

            home_dir: dirs.home_dir,
            app_dir: dirs.app_dir,
            network_dir: dirs.network_dir,
            db_dir: dirs.db_dir,
            utxo_index_db_dir: dirs.utxo_index_db_dir,
            meta_db_dir: dirs.meta_db_dir,
            consensus_db_dir: dirs.consensus_db_dir
        }
    }
    
    pub fn get_cs(&self) -> u64 {
        // TODO check if utxoindex exists on system first

        self.store_manager.circulating_supply_store.as_ref().unwrap().get().unwrap()
    }

    pub fn get_utxo_tips(&self) -> HashSet<String> {
        // TODO check if utxoindex exists on system first

        // Get tips from store
        let utxo_tips = self.store_manager.utxo_tips_store.as_ref().unwrap().get().unwrap();

        // Return as HashSet<String> (rather than BlockHashSet) for ease of type conversion w/ PyO3
        let mut tips = HashSet::new();
        for tip in utxo_tips.iter() {
            tips.insert(tip.to_string());
        }

        tips
    }

    // pub fn export_utxos() -> Result<PathBuf, E> {
        // Export UTXO set to a CSV file
    // }

    // pub fn get_utxos()
        // loads all utxos into memory and returns
        // would be nice if there was a way to chunk and yield like a python generator

        // param to control pubkey script (exclude, as adress, bytes, hex, etc.)
        // param to control transaction_outpoint (exclude, etc.)
        // param to include/exclude daa
        // param to include/exclude amount
        // param to include/exclude is_coinbase

    // get utxos by script

    // get unique addresses
    // export unique addresses 

    // get utxo ages
    // export utxo ages
}
