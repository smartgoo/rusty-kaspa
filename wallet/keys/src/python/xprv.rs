use crate::python::derivation_path::DerivationPath;
use kaspa_bip32::{ChildNumber, ExtendedPrivateKey, ExtendedPublicKey, SecretKey};
use kaspa_utils::hex::*;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::str::FromStr;

#[pyclass]
pub struct XPrv {
    inner: ExtendedPrivateKey<SecretKey>
}

impl XPrv {
    pub fn inner(&self) -> &ExtendedPrivateKey<SecretKey> {
        &self.inner
    }
}

#[pymethods]
impl XPrv {
    #[new]
    pub fn try_new(seed: String) -> PyResult<XPrv> {
        let seed_bytes = Vec::<u8>::from_hex(&seed).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?; // TODO get rid of map_err

        let inner = ExtendedPrivateKey::<SecretKey>::new(seed_bytes)?;
        Ok(Self { inner })
    }

    #[pyo3(name = "from_xprv")]
    #[staticmethod]
    pub fn from_xprv_str(xprv: String) -> PyResult<XPrv> {
        let inner = ExtendedPrivateKey::<SecretKey>::from_str(&xprv)?;
        Ok(Self { inner })
    }

    // #[staticmethod] // TODO
    pub fn derive_child(&self, child_number: u32, hardened: Option<bool>) -> PyResult<XPrv> {
        let child_number = ChildNumber::new(child_number, hardened.unwrap_or(false))?;
        let inner = self.inner.derive_child(child_number)?;
        Ok(Self { inner })
    }

    // #[staticmethod] // TODO
    pub fn derive_path(&self, path: &str) -> PyResult<XPrv> {
        let path = DerivationPath::new(path)?; // TODO is this correct?
        let inner = self.inner.clone().derive_path((&path).into())?;
        Ok(Self { inner })
    }

    pub fn into_string(&self, prefix: &str) -> PyResult<String> {
        let str = self.inner.to_extended_key("kprv".try_into()?).to_string();
        Ok(str)
    }

    pub fn to_string(&self) -> PyResult<String> { 
        let str = self.inner.to_extended_key("kprv".try_into()?).to_string();
        Ok(str)
    }

    // pub fn to_xpub() -> PyResult<XPub> {} TODO once XPub done
}