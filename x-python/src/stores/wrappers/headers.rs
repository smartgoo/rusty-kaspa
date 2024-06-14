use crate::converters::ToPyDict;
use kaspa_consensus::model::stores::headers::{DbHeadersStore, HeaderStoreReader};
use kaspa_consensus_core::Hash;
use kaspa_database::prelude::{CachePolicy, StoreError, DB};
use pyo3::prelude::*;
use std::{str::FromStr, sync::Arc};

#[derive(Clone)]
#[pyclass]
#[pyo3(name = "HeadersStore")]
pub struct PyHeaderStore {
    // To avoid Rust -> Python type conversion fun, not exposing this to Python
    inner_store: DbHeadersStore,
}

impl PyHeaderStore {
    pub fn new(consensus_db: Arc<DB>) -> Self {
        let inner_store = DbHeadersStore::new(consensus_db, CachePolicy::Empty, CachePolicy::Empty);

        PyHeaderStore { inner_store }
    }
}

#[pymethods]
impl PyHeaderStore {
    pub fn get(&self, py: Python, block_hash: String) -> PyResult<PyObject> {
        let block_hash = Hash::from_str(&block_hash).unwrap();
    
        match self.inner_store.get_header(block_hash) {
            Ok(header) => Ok(header.to_py_dict(py).to_object(py)),
            Err(StoreError::KeyNotFound(_)) => Ok(py.None()),
            _ => todo!(),
        }
    }
}

// Gets block header for given hash from consensus DB. Returns a dict
// pub fn get_block(&self, py: Python, block_hash: String, include_transactions: bool) -> PyResult<PyObject> {
//     // TODO checkout consensus/src/consensus/mod.rs:636
    
//     // Convert block_hash from String to Hash
//     let block_hash = Hash::from_str(&block_hash).unwrap();

//     // Get header
//     let header = match self.stores.headers_store.get_header(block_hash) {
//         Ok(header) => Some(header),
//         Err(StoreError::KeyNotFound(_)) => None,
//         _ => todo!(), // TODO rest of potential StoreErrors
//     };

//     // If header is None, block isn't found. Return
//     if header.is_none() {
//         return Ok(py.None());
//     }

//     // If transactions param is true, attempt to get transactions
//     let transactions = if include_transactions {
//         match self.stores.block_transactions_store.get(block_hash) {
//             Ok(transactions) => Some(transactions),
//             Err(StoreError::KeyNotFound(_)) => None,
//             _ => todo!(), // TODO rest of potential StoreErrors
//         }
//     } else {
//         None
//     };

//     let unwrapped_header = header.unwrap();
//     let header_dict = unwrapped_header.to_py_dict(py);

//     // Build return dict
//     let dict = PyDict::new(py);
//     dict.set_item("header", header_dict).unwrap();

//     // Convert transactions to Python dict
//     if include_transactions {
//         let transaction_list = transactions_to_py_list(py, (&transactions.unwrap()).to_vec());
//         dict.set_item("transactions", transaction_list).unwrap();
//     }

//     Ok(dict.to_object(py))
// }