use crate::python::derivation_path::DerivationPath;
use crate::python::publickey::PublicKey;
use kaspa_bip32::{ChildNumber, ExtendedPublicKey};
use pyo3::prelude::*;
use std::str::FromStr;

#[derive(Clone)]
#[pyclass]
pub struct XPub {
    inner: ExtendedPublicKey<secp256k1::PublicKey>,
}

impl XPub {
    pub fn inner(&self) -> &ExtendedPublicKey<secp256k1::PublicKey> {
        &self.inner
    }
}

#[pymethods]
impl XPub {
    // TODO static method?
    #[new]
    pub fn try_new(xpub: &str) -> PyResult<XPub> {
        let inner = ExtendedPublicKey::<secp256k1::PublicKey>::from_str(xpub)?;
        Ok(Self { inner })
    }

    pub fn derive_child(&self, child_number: u32, hardened: Option<bool>) -> PyResult<XPub> {
        let child_number = ChildNumber::new(child_number, hardened.unwrap_or(false))?;
        let inner = self.inner.derive_child(child_number)?;
        Ok(Self { inner })
    }

    pub fn derive_path(&self, path: &str) -> PyResult<XPub> {
        let path = DerivationPath::new(&path)?;
        let inner = self.inner.clone().derive_path((&path).into())?;
        Ok(Self { inner })
    }

    #[pyo3(name = "into_string")]
    pub fn to_str(&self, prefix: &str) -> PyResult<String> {
        Ok(self.inner.to_string(Some(prefix.try_into()?)))
    }

    pub fn public_key(&self) -> PublicKey {
        self.inner.public_key().into()
    }
}

impl From<ExtendedPublicKey<secp256k1::PublicKey>> for XPub {
    fn from(inner: ExtendedPublicKey<secp256k1::PublicKey>) -> Self {
        Self { inner }
    }
}