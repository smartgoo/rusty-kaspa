use crate::stores::indexed_utxos::{DbUtxoSetByScriptPublicKeyStore};

use kaspa_database::prelude::DB;
use kaspa_utxoindex::model::{CompactUtxoEntry};
use pyo3::prelude::*;
use std::{sync::Arc};
use std::error::Error;

#[derive(Clone)]
#[pyclass]
#[pyo3(name = "UtxoIndexStore")]
pub struct PyUtxoIndexStore {
    // To avoid Rust -> Python type conversion fun, not exposing this to Python
    db: Arc<DB>,
    inner_store: DbUtxoSetByScriptPublicKeyStore,
}

impl PyUtxoIndexStore {
    pub fn new(utxo_index_db: Arc<DB>) -> Self {
        // Init inner store
        let inner_store = DbUtxoSetByScriptPublicKeyStore::new(utxo_index_db.clone(), 0);

        PyUtxoIndexStore {
            db: utxo_index_db,
            inner_store
        }
    }
}

#[pymethods]
impl PyUtxoIndexStore {
    /// Exports entire UTXO set to a CSV file. Returns count of UTXOs exported.
    #[pyo3(signature = (
        filepath,
        address=true,
        daa_score=true,
        amount=true,
        is_coinbase=true,
        outpoint=false,
        chunk_size=100000,
        verbose=false
    ))]
    pub fn export(
        &self,
        filepath: String,
        address: bool,
        daa_score: bool,
        amount: bool,
        is_coinbase: bool,
        outpoint: bool,
        chunk_size: i32,
        verbose: bool,
    ) -> PyResult<i64> {
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

    #[pyo3(text_signature = "()")]
    pub fn iterate(&self, py: Python) -> PyResult<Py<PyUtxoIterator>> {
        let iterator = self.inner_store.get_utxo_set_iterator(); // Method to get the iterator
        let utxo_iter = PyUtxoIterator { inner: iterator };
        Py::new(py, utxo_iter)
    }
}

#[pyclass]
struct PyUtxoIterator {
    inner: Box<dyn Iterator<Item = Result<(Box<[u8]>, CompactUtxoEntry), Box<dyn Error>>> + Send>
}

#[pymethods]
impl PyUtxoIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyUtxoEntry> {
        slf.inner.next().map(|utxo| {
            let (key, value): (Box<[u8]>, CompactUtxoEntry) = utxo.unwrap();

            PyUtxoEntry {
                // transaction_id: utxo.transaction_id.to_string(),
                // index: utxo.index,
                // address: utxo.address,
                daa_score: value.block_daa_score,
                amount: value.amount,
                is_coinbase: value.is_coinbase,
            }
        })
    }
}

#[pyclass]
pub struct PyUtxoEntry {
    // transaction_id: String,
    // index: u8,
    // address: String,
    daa_score: u64,
    amount: u64,
    is_coinbase: bool,
}