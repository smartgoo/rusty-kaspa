mod converters;
mod core;
mod stores;

use pyo3::prelude::*;

#[pymodule]
fn kaspadbr(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Classes
    m.add_class::<core::reader::Reader>()?;

    Ok(())
}