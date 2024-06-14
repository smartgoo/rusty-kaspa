pub mod block_header;
pub mod transaction;
mod types;

use pyo3::prelude::*;
use pyo3::types::PyDict;

pub trait ToPyDict {
    fn to_py_dict<'a>(&'a self, py: Python<'a>) -> &'a PyDict;
}