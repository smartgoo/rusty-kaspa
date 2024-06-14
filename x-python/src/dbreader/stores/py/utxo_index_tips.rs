use kaspa_database::prelude::DB;
use kaspa_utxoindex::stores::tips::{DbUtxoIndexTipsStore, UtxoIndexTipsStoreReader};
use pyo3::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
#[pyclass]
#[pyo3(name = "UtxoIndexTipsStore")]
pub struct PyUtxoIndexTipsStore {
    inner_store: DbUtxoIndexTipsStore,
}

impl PyUtxoIndexTipsStore {
    pub fn new(utxo_index_db: Arc<DB>) -> Self {
        // Init inner store
        let inner_store = DbUtxoIndexTipsStore::new(utxo_index_db.clone());

        PyUtxoIndexTipsStore { inner_store }
    }   
}

#[pymethods]
impl PyUtxoIndexTipsStore {
    pub fn get(&self) -> PyResult<Vec<String>> {
        // Get tips from store
        let utxo_tips = self.inner_store.get().unwrap();

        // Return as Vec<String> (rather than BlockHashSet) for ease of type conversion w/ PyO3
        let mut tips = Vec::new();
        for tip in utxo_tips.iter() {
            tips.push(tip.to_string());
        }

        Ok(tips)
    }
}