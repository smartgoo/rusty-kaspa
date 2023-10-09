mod core;
mod reader;
mod stores;

use pyo3::prelude::*;

#[pymodule]
fn rusty_kaspa_db(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Classes
    m.add_class::<reader::Reader>()?;
    // m.add_class::<stores::utxoindex::tips::DbUtxoIndexTipsStore>()?;

    // Functions
    // m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}