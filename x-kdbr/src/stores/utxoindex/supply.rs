use std::{
    sync::Arc,
};

use kaspa_database::{
    prelude::{
        DB,
        CachedDbItem,
        StoreResult,
    },
    registry::DatabaseStorePrefixes
};

/*
    Source used for this file:
    rusty-kaspa/indexes/utxoindex/src/stores/supply.rs
*/

pub struct DbCirculatingSupplyStore {
    db: Arc<DB>,
    access: CachedDbItem<u64>,
}

impl DbCirculatingSupplyStore {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db: Arc::clone(&db), access: CachedDbItem::new(db, DatabaseStorePrefixes::CirculatingSupply.into()) }
    }

    pub fn get(&self) -> StoreResult<u64> {
        self.access.read()
    }
}