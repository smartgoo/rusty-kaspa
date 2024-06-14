use crate::core::dirs::Dirs;
use crate::stores::wrappers::{
    circulating_supply::PyCirculatingSupplyStore,
    headers::PyHeaderStore,
    metadata::PyMetadataStore,
    utxo_index::PyUtxoIndexStore,
    utxo_index_tips::PyUtxoIndexTipsStore,
};

use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass]
pub struct Stores {
    #[pyo3(get)]
    metadata: PyMetadataStore,

    #[pyo3(get)]
    headers: PyHeaderStore,

    #[pyo3(get)]
    circulating_supply: Option<PyCirculatingSupplyStore>,

    #[pyo3(get)]
    utxo_index: Option<PyUtxoIndexStore>,

    #[pyo3(get)]
    utxo_index_tips: Option<PyUtxoIndexTipsStore>,
}

impl Stores {
    pub fn new(dirs: &Dirs) -> PyResult<Self> {
        // Construct meta DB and store
        let meta_db = kaspa_database::prelude::ConnBuilder::default()
            .with_db_path(dirs.meta_db_dir.clone())
            .with_files_limit(10) // TODO file limit
            .build_readonly()
            .unwrap();
        let metadata = PyMetadataStore::new(meta_db);

        // Construct active consensus DB
        let current_consensus_key = metadata.current_consensus_key().unwrap();
        let consensus_db = kaspa_database::prelude::ConnBuilder::default()
            .with_db_path(dirs.consensus_db_dir.join(format!("consensus-{:0>3}", current_consensus_key)))
            .with_files_limit(10) // TODO file limit
            .build_readonly()
            .unwrap();

        // Headers store
        let headers = PyHeaderStore::new(consensus_db);

        // Construct all utxo index stores, if utxoindex dir exists
        let mut circulating_supply: Option<PyCirculatingSupplyStore> = None;
        let mut utxo_index: Option<PyUtxoIndexStore> = None;
        let mut utxo_index_tips: Option<PyUtxoIndexTipsStore> = None;

        if dirs.utxo_index_db_dir.is_some() {
            // Create utxo index db
            let utxo_index_db = kaspa_database::prelude::ConnBuilder
                ::default()
                .with_db_path(dirs.utxo_index_db_dir.clone().unwrap())
                .with_files_limit(10) // TODO file limit
                .build_readonly()
                .unwrap();

            // Create UTXO Stores
            circulating_supply = Some(PyCirculatingSupplyStore::new(utxo_index_db.clone()));
            utxo_index = Some(PyUtxoIndexStore::new(utxo_index_db.clone()));
            utxo_index_tips = Some(PyUtxoIndexTipsStore::new(utxo_index_db.clone()));
        }

        Ok(Stores {
            metadata, 
            headers,
            circulating_supply,
            utxo_index,
            utxo_index_tips,
        })
    }
}