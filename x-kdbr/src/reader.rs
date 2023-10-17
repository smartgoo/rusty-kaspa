use std::{
    collections::HashSet,
    path::PathBuf,
};
use pyo3::prelude::*;
use crate::core::{
    dirs::Dirs, 
    stores::Stores
};

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
            consensus_db_dir: dirs.consensus_db_dir
        }
    }
    
    pub fn get_cs(&self) -> u64 {
        // TODO ensure utxoindex dir exists first

        self.stores.circulating_supply_store.as_ref().unwrap().get().unwrap()
    }

    pub fn get_utxo_tips(&self) -> HashSet<String> {
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

    pub fn export_all_outpoints(&self) -> String {
        /* Need to add these params:
            chunk_size:
                - default: 100,000
            verbose:
                - options: true, false
                - default: false
                - if true, function prints counts exported as it goes
            script_public_key:
                - how to export script pub key
                - options: none, hex encoded, address
                - default: address
            daa_score:
                - export daa score?
                - options: true, false
                - default: true
            value:
                - how to export value
                - options: none, kas, sompi
                - default: sompi
            is_coinbase:
                - export is_coinbase?
                - options: true, false
                - default: true

            outpoint:
                - export outpoint (tx id and index?)?
                - options: true, false
                - default ?

            daa_timestamp:
                - include column w/ daa_timestamp
                - options: true, false
                - default: false

            should this support any other format than csv? json? xml?
        */
        self.stores.utxo_store.as_ref().unwrap().export_all_outpoints()
    }

    // fn get_utxos() - all in memory
    // fn get_utxos_by_address()
    // fn get_unique_addresses() - param to include balances
    // fn get_export_unique_addresses()
}
