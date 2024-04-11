use secp256k1::{Secp256k1};

use pyo3::prelude::*;

#[pyclass]
pub struct PrivateKey {
    inner: secp256k1::SecretKey,
}

impl PrivateKey {
    pub fn secret_bytes(&self) -> [u8; 32] {
        self.inner.secret_bytes()
    }
}

impl From<&secp256k1::SecretKey> for PrivateKey {
    fn from(value: &secp256k1::SecretKey) -> Self {
        Self { inner: *value }
    }
}

impl PrivateKey {
    #[new]
    pub fn try_new(key: &str) -> PyResult<PrivateKey> {
        Ok(Self { inner: secp256k1::SecretKey::from_str(key)? })
    }
}