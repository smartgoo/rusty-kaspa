use kaspa_database::prelude::DB;
use kaspa_utxoindex::stores::{
    supply::{CirculatingSupplyStoreReader, DbCirculatingSupplyStore}, 
};
use pyo3::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
#[pyclass]
#[pyo3(name = "CirculatingSupplyStore")]
pub struct PyCirculatingSupplyStore {
    // To avoid Rust -> Python type conversion fun, not exposing this to Python
    inner_store: DbCirculatingSupplyStore,
}

impl PyCirculatingSupplyStore {
    pub fn new(utxo_index_db: Arc<DB>) -> Self {
        let inner_store = DbCirculatingSupplyStore::new(utxo_index_db);

        PyCirculatingSupplyStore {
            inner_store,
        }
    }
}

#[pymethods]
impl PyCirculatingSupplyStore {
    pub fn get(&self) -> PyResult<u64> {
        Ok(self.inner_store.get().unwrap())
    }
}