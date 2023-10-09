use crate::core::{dir_manager::DirManager};
use crate::stores::{
    utxoindex::{
        supply::DbCirculatingSupplyStore,
        tips::DbUtxoIndexTipsStore,
    },
};

pub struct StoreManager {
    pub circulating_supply_store: Option<DbCirculatingSupplyStore>,
    pub utxo_tips_store: Option<DbUtxoIndexTipsStore>,
}

impl StoreManager {
    pub fn new(dirs: &DirManager) -> Self {

        // Create all utxo index stores, if utxoindex dir exists
        let mut circulating_supply_store: Option<DbCirculatingSupplyStore> = None;
        let mut utxo_tips_store: Option<DbUtxoIndexTipsStore> = None;
        if dirs.utxo_index_db_dir.is_some() {

            // Create utxo index db
            let utxo_index_db = kaspa_database::prelude::ConnBuilder
                ::default()
                .with_db_path(dirs.utxo_index_db_dir.clone().unwrap())
                .build();

            // Create circulating supply store
            circulating_supply_store = Some(DbCirculatingSupplyStore::new(utxo_index_db.clone())); // TODO is it right to clone?

            // Create utxo tips store
            utxo_tips_store = Some(DbUtxoIndexTipsStore::new(utxo_index_db.clone())); // TODO is it right to clone?
        }

        StoreManager {
            circulating_supply_store,
            utxo_tips_store,
        }
    }
}
