use crate::{ChildNumber};
use std::str::FromStr;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass]
pub struct DerivationPath {
    inner: crate::DerivationPath,
}

#[pymethods]
impl DerivationPath {
    #[new]
    pub fn new(path: &str) -> PyResult<DerivationPath> {
        let inner = crate::DerivationPath::from_str(path).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
        Ok(Self { inner })
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn parent(&self) -> Option<DerivationPath> {
        self.inner.parent().map(|inner| Self { inner })
    }

    pub fn push(&mut self, child_number: u32, hardened: Option<bool>) -> PyResult<()> {
        let child = ChildNumber::new(child_number, hardened.unwrap_or(false)).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
        self.inner.push(child);
        Ok(())
    }

    #[pyo3(name = "to_string")]
    pub fn to_str(&self) -> String {
        self.inner.to_string()
    }
}

impl From<DerivationPath> for crate::DerivationPath {
    fn from(value: DerivationPath) -> Self {
        value.inner
    }
}