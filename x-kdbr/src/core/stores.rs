use crate::core::{dirs::Dirs};
use crate::stores::{
    utxoindex::{
        supply::DbCirculatingSupplyStore,
        tips::DbUtxoIndexTipsStore,
        indexed_utxos::DbUtxoSetByScriptPublicKeyStore,
    },
};

pub struct Stores {
    pub circulating_supply_store: Option<DbCirculatingSupplyStore>,
    pub utxo_tips_store: Option<DbUtxoIndexTipsStore>,
    pub utxo_store: Option<DbUtxoSetByScriptPublicKeyStore>,
}

impl Stores {
    pub fn new(dirs: &Dirs) -> Self {

        // Create all utxo index stores, if utxoindex dir exists
        let mut circulating_supply_store: Option<DbCirculatingSupplyStore> = None;
        let mut utxo_tips_store: Option<DbUtxoIndexTipsStore> = None;
        let mut utxo_store: Option<DbUtxoSetByScriptPublicKeyStore> = None;
        
        if dirs.utxo_index_db_dir.is_some() {

            // Create utxo index db
            let utxo_index_db = kaspa_database::prelude::ConnBuilder
                ::default()
                .with_db_path(dirs.utxo_index_db_dir.clone().unwrap())
                .build();

            // Create UTXO Stores
            circulating_supply_store = Some(DbCirculatingSupplyStore::new(utxo_index_db.clone())); // TODO is it right to clone?
            utxo_tips_store = Some(DbUtxoIndexTipsStore::new(utxo_index_db.clone())); // TODO is it right to clone?
            utxo_store = Some(DbUtxoSetByScriptPublicKeyStore::new(utxo_index_db.clone(), 0));
        }

        Stores {
            circulating_supply_store,
            utxo_tips_store,
            utxo_store,
        }
    }
}
