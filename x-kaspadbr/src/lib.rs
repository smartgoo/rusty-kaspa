mod converters;
mod core;
mod interface;
mod stores;

use pyo3::prelude::*;

#[pymodule]
fn kaspadbr(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Old main Reader class
    m.add_class::<core::reader::Reader>()?;

    // Classes that are 1:1 with a rusty-kaspa stores.
    let store_submodule = PyModule::new(py, "stores")?;
    store_submodule.add_class::<interface::circulating_supply_store::PyCirculatingSupplyStore>()?;
    store_submodule.add_class::<interface::utxo_index_store::PyUtxoIndexStore>()?;
    store_submodule.add_class::<interface::utxo_index_tips_store::PyUtxoIndexTipsStore>()?;

    m.add_submodule(store_submodule)?;
    
    Ok(())
}