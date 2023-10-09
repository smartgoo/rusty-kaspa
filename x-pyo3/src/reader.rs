use std::{
    collections::HashSet,
    fs,
    path::PathBuf,
    sync::Arc,

};

use pyo3::prelude::*;

use kaspa_consensus_core::BlockHashSet;
use kaspa_database::prelude::DB;

use crate::stores::{
    utxoindex::{
        supply::DbCirculatingSupplyStore,
        tips::DbUtxoIndexTipsStore,
    },
};


pub fn get_home_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    return dirs::data_local_dir().unwrap();
    #[cfg(not(target_os = "windows"))]
    return dirs::home_dir().unwrap();
}

pub fn get_app_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    return get_home_dir().join("rusty-kaspa");
    #[cfg(not(target_os = "windows"))]
    return get_home_dir().join(".rusty-kaspa");
}

pub fn get_out_dir() -> PathBuf {
    let outdir = get_home_dir().join("rusty-kaspa-out");
    if !outdir.exists() {
        let _ = fs::create_dir_all(&outdir);
    }
    return outdir;
}


struct StoreManager {
    circulating_supply_store: Option<DbCirculatingSupplyStore>,
    utxo_tips_store: Option<DbUtxoIndexTipsStore>,
}

#[pyclass]
pub struct Reader {
    stores: StoreManager,

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
        // TODO check that DB exists at some point in this function

        // Set home_dir based on users OS
        let home_dir = get_home_dir();

        // Set app_dir. Use passed param, if one exists, or default app dir
        let app_dir = match app_dir {
            Some(dir) => dir,
            None => get_app_dir(),
        };

        // Set network_dir based on passed network
        let network_dir = match network.as_deref() {
            Some("mainnet") => app_dir.join("kaspa-mainnet"),
            Some("testnet") => app_dir.join("kaspa-testnet"),
            Some("devnet") => app_dir.join("kaspa-devnet"),
            Some("simnet") => app_dir.join("kaspa-simnet"),
            _ => app_dir.join("kaspa-mainnet"),
        };

        // Set db_dir
        let db_dir = network_dir.join("datadir");

        // Set utxo_index_db_dir if utxoindex dir exists inside of db_dir
        let utxo_index_db_dir = if db_dir.join("utxoindex").exists() {
            Some(db_dir.join("utxoindex"))
        } else {
            None
        };

        // Create all utxo index stores, if utxoindex dir exists
        let mut circulating_supply_store: Option<DbCirculatingSupplyStore> = None;
        let mut utxo_tips_store: Option<DbUtxoIndexTipsStore> = None;
        if utxo_index_db_dir.is_some() {

            // Create utxo index db
            let utxo_index_db = kaspa_database::prelude::ConnBuilder
                ::default()
                .with_db_path(utxo_index_db_dir.clone().unwrap())
                .build();

            // Create circulating supply store
            circulating_supply_store = Some(DbCirculatingSupplyStore::new(utxo_index_db.clone())); // TODO is it right to clone?

            // Create utxo tips store
            utxo_tips_store = Some(DbUtxoIndexTipsStore::new(utxo_index_db.clone())); // TODO is it right to clone?
        }

        // Set meta_db_dir
        let meta_db_dir = db_dir.join("meta");

        // Set consensus_db_dir
        let consensus_db_dir = db_dir.join("consensus");

        let stores = StoreManager {
            circulating_supply_store,
            utxo_tips_store
        };

        Reader {
            stores,

            home_dir,
            app_dir,
            network_dir,
            db_dir,
            utxo_index_db_dir,
            meta_db_dir,
            consensus_db_dir
        }
    }
    
    pub fn get_cs(&self) -> u64 {
        // TODO check if utxoindex exists on system first

        self.stores.circulating_supply_store.as_ref().unwrap().get().unwrap()
    }

    pub fn get_utxo_tips(&self) -> HashSet<String> {
        // TODO check if utxoindex exists on system first

        // Get tips from store
        let utxo_tips = self.stores.utxo_tips_store.as_ref().unwrap().get().unwrap();

        // Return as HashSet<String> (rather than BlockHashSet) for ease of type conversion w/ PyO3
        let mut tips = HashSet::new();
        for tip in utxo_tips.iter() {
            tips.insert(tip.to_string());
        }

        tips
    }

    // pub fn get_utxos()
        // loads all utxos into memory and returns
        // would be nice if there was a way to chunk and yield like a python generator

        // param to control pubkey script (exclude, as adress, bytes, hex, etc.)
        // param to control transaction_outpoint (exclude, etc.)
        // param to include/exclude daa
        // param to include/exclude amount
        // param to include/exclude is_coinbase

    // pub fn export_utxos()
        // export utxos to csv

    // get utxos by script

    // get unique addresses
    // export unique addresses 

    // get utxo ages
    // export utxo ages
}
