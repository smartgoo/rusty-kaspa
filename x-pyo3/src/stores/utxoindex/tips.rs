use std::sync::Arc;

use kaspa_consensus_core::BlockHashSet;
use kaspa_database::{
    prelude::{CachedDbItem, DB, StoreResult},
    registry::DatabaseStorePrefixes,
};

/*
    Source used for this file:
    rusty-kaspa/indexes/utxoindex/src/stores/tips.rs
*/

pub struct DbUtxoIndexTipsStore {
    db: Arc<DB>,
    access: CachedDbItem<Arc<BlockHashSet>>,
}

impl DbUtxoIndexTipsStore {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db: Arc::clone(&db), access: CachedDbItem::new(db.clone(), DatabaseStorePrefixes::UtxoIndexTips.into()) }
    }

    pub fn get(&self) -> StoreResult<Arc<BlockHashSet>> {
        self.access.read()
    }
}