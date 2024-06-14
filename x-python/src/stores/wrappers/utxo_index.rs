use crate::stores::extensions::indexed_utxos::UtxoByScriptPublicKeyStoreExt;
use kaspa_database::prelude::{DB, CachePolicy};
use kaspa_utxoindex::stores::indexed_utxos::DbUtxoSetByScriptPublicKeyStore;
use pyo3::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
#[pyclass]
#[pyo3(name = "UtxoIndexStore")]
pub struct PyUtxoIndexStore {
    inner_store: DbUtxoSetByScriptPublicKeyStore,
}

impl PyUtxoIndexStore {
    pub fn new(utxo_index_db: Arc<DB>) -> Self {
        let inner_store = 
            DbUtxoSetByScriptPublicKeyStore::new(utxo_index_db.clone(), CachePolicy::Empty);
    
        Self {
            inner_store
        }
    }
}

#[pymethods]
impl PyUtxoIndexStore {
    #[pyo3(signature = (filepath, address=true, daa_score=true, amount=true, is_coinbase=true, outpoint=false, chunk_size=100000, verbose=false))]
    pub fn export(&self, filepath: String, address: bool, daa_score: bool, amount: bool, is_coinbase: bool, outpoint: bool, chunk_size: i32, verbose: bool) -> PyResult<i64> {
        Ok(self.inner_store.export_all_outpoints(
            filepath,
            address,
            daa_score,
            amount,
            is_coinbase,
            outpoint,
            chunk_size,
            verbose,
        ))
    }

    #[pyo3(signature = (filepath, chunk_size=100000, verbose=false))]
    pub fn export_addresses(&self, filepath: String, chunk_size: i32, verbose: bool) -> PyResult<i64> {
        Ok(self.inner_store.export_all_addresses(filepath, chunk_size, verbose))
    }
}