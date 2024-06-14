use kaspa_consensus::consensus::factory::MultiConsensusManagementStore;

pub trait MultiConsensusManagementStoreExt {
    fn get_current_consensus_entry(&self) -> Option<u64>;
}

impl MultiConsensusManagementStoreExt for MultiConsensusManagementStore {
    fn get_current_consensus_entry(&self) -> Option<u64> {
        let metadata = self.metadata.read().ok()?;
        metadata.current_consensus_key
    }
}