mod converters;
mod core;
mod stores;

use pyo3::prelude::*;

#[pymodule]
fn kaspadbr(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<core::db_reader::DBReader>()?;

    Ok(())
}