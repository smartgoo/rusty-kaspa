use kaspa_bip32::{ChildNumber};
use std::str::FromStr;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass]
pub struct DerivationPath {
    inner: kaspa_bip32::DerivationPath,
}

#[pymethods]
impl DerivationPath {
    #[new]
    pub fn new(path: &str) -> PyResult<DerivationPath> {
        let inner = kaspa_bip32::DerivationPath::from_str(path)?;
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
        let child = ChildNumber::new(child_number, hardened.unwrap_or(false))?;
        self.inner.push(child);
        Ok(())
    }

    #[pyo3(name = "to_string")]
    pub fn to_str(&self) -> String {
        self.inner.to_string()
    }
}

impl<'a> From<&'a DerivationPath> for &'a kaspa_bip32::DerivationPath {
    fn from(value: &'a DerivationPath) -> Self {
        &value.inner
    }
}