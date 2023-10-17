mod core;
mod reader;
mod stores;

use pyo3::prelude::*;

#[pymodule]
fn kdbr(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Classes
    m.add_class::<reader::Reader>()?;

    Ok(())
}