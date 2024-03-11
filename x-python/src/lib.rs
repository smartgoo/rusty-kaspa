// mod converters;
mod dbreader;
// mod py_stores;
// mod stores;

use pyo3::prelude::*;

#[pymodule]
fn kaspapy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<dbreader::core::db_reader::DBReader>()?;
    
    Ok(())
}