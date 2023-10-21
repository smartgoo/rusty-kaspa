use kaspa_consensus::{consensus::factory::ConsensusEntry, model::stores::U64Key};
use kaspa_database::{
    prelude::{CachedDbAccess, CachedDbItem, DB},
    registry::DatabaseStorePrefixes,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

/*
    Source used for this file:
    rusty-kaspa/consensus/src/consensus/factory.rs
*/

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct MultiConsensusMetadata {
    current_consensus_key: Option<u64>,
    staging_consensus_key: Option<u64>,
    /// Max key used for a consensus entry
    max_key_used: u64,
    /// Memorizes whether this node was recently an archive node
    is_archival_node: bool,
    /// General serialized properties to be used cross DB versions
    props: HashMap<Vec<u8>, Vec<u8>>,
    /// The DB scheme version
    version: u32,
}

#[derive(Clone)]
pub struct MultiConsensusManagementStore {
    db: Arc<DB>,
    entries: CachedDbAccess<U64Key, ConsensusEntry>,
    metadata: CachedDbItem<MultiConsensusMetadata>,
}

impl MultiConsensusManagementStore {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db: db.clone(),
            entries: CachedDbAccess::new(db.clone(), 16, DatabaseStorePrefixes::ConsensusEntries.into()),
            metadata: CachedDbItem::new(db, DatabaseStorePrefixes::MultiConsensusMetadata.into()),
        }
    }

    pub fn get_current_consensus_entry(&self) -> Option<u64> {
        let metadata = self.metadata.read().ok()?;
        metadata.current_consensus_key
    }
}
