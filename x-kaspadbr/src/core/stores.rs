use crate::core::dirs::Dirs;
use crate::stores::{
    meta::MultiConsensusManagementStore,
    indexed_utxos::DbUtxoSetByScriptPublicKeyStore,
};
use kaspa_consensus::model::stores::{block_transactions::DbBlockTransactionsStore, headers::DbHeadersStore, tips::DbTipsStore};
// use kaspa_utxoindex::stores::{tips::DbUtxoIndexTipsStore};

pub struct Stores {
    pub meta_store: MultiConsensusManagementStore,

    // Consensus stores
    pub tips_store: DbTipsStore,
    pub headers_store: DbHeadersStore,
    pub block_transactions_store: DbBlockTransactionsStore,

    // UTXO stores
    // pub circulating_supply_store: Option<DbCirculatingSupplyStore>,
    // pub utxo_tips_store: Option<DbUtxoIndexTipsStore>,
    pub utxo_store: Option<DbUtxoSetByScriptPublicKeyStore>,
}

impl Stores {
    pub fn new(dirs: &Dirs) -> Self {
        // Construct meta DB and store
        let meta_db = kaspa_database::prelude::ConnBuilder::default()
            .with_db_path(dirs.meta_db_dir.clone())
            .with_files_limit(10) // TODO
            .build()
            .unwrap();
        let meta_store = MultiConsensusManagementStore::new(meta_db);

        // Construct active consensus DB
        let current_consensus_key = meta_store.get_current_consensus_entry().unwrap();
        let consensus_db = kaspa_database::prelude::ConnBuilder::default()
            .with_db_path(dirs.consensus_db_dir.join(format!("consensus-{:0>3}", current_consensus_key)))
            .with_files_limit(10) // TODO
            .build()
            .unwrap();

        // Construct consensus stores
        let tips_store = DbTipsStore::new(consensus_db.clone());
        let headers_store = DbHeadersStore::new(consensus_db.clone(), 0);
        let block_transactions_store = DbBlockTransactionsStore::new(consensus_db.clone(), 0);

        // Construct all utxo index stores, if utxoindex dir exists
        // let mut circulating_supply_store: Option<DbCirculatingSupplyStore> = None;
        // let mut utxo_tips_store: Option<DbUtxoIndexTipsStore> = None;
        let mut utxo_store: Option<DbUtxoSetByScriptPublicKeyStore> = None;
        
        if dirs.utxo_index_db_dir.is_some() {
            // Create utxo index db
            let utxo_index_db = kaspa_database::prelude::ConnBuilder
                ::default()
                .with_db_path(dirs.utxo_index_db_dir.clone().unwrap())
                .with_files_limit(10) // TODO
                .build()
                .unwrap();

            // Create UTXO Stores
            // circulating_supply_store = Some(DbCirculatingSupplyStore::new(utxo_index_db.clone()));
            // utxo_tips_store = Some(DbUtxoIndexTipsStore::new(utxo_index_db.clone()));
            utxo_store = Some(DbUtxoSetByScriptPublicKeyStore::new(utxo_index_db.clone(), 0));
        }

        Stores {
            meta_store,

            tips_store,
            headers_store,
            block_transactions_store,

            // circulating_supply_store,
            // utxo_tips_store,
            utxo_store,
        }
    }
}
