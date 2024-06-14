use crate::stores::extensions::metadata::MultiConsensusManagementStoreExt;
use kaspa_consensus::consensus::factory::MultiConsensusManagementStore;
use kaspa_database::prelude::DB;
use pyo3::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
#[pyclass]
#[pyo3(name = "MetadataStore")]
pub struct PyMetadataStore { 
    // To avoid Rust -> Python type conversion fun, not exposing this to Python
    inner_store: MultiConsensusManagementStore,
}

impl PyMetadataStore {
    pub fn new(meta_db: Arc<DB>) -> Self {
        let inner_store = MultiConsensusManagementStore::new(meta_db);

        PyMetadataStore { inner_store }
    }
}

#[pymethods]
impl PyMetadataStore {
    pub fn current_consensus_key(&self) -> PyResult<u64> {
        Ok(self.inner_store.get_current_consensus_entry().unwrap())
    }
}