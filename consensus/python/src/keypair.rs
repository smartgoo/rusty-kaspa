use secp256k1::{Secp256k1, XOnlyPublicKey};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use pyo3::exceptions::PyException;
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

#[pymethods]
impl PrivateKey {
    #[new]
    pub fn try_new(key: &str) -> PyResult<Self> {
        let secret_key = secp256k1::SecretKey::from_str(key).map_err(|e| {
            PyErr::new::<PyException, _>(format!("Failed to parse secret key: {}", e))
        })?;
        
        Ok(Self { inner: secret_key })
    }

    #[pyo3(name = "to_string")]
    pub fn to_hex(&self) -> String {
        use kaspa_utils::hex::ToHex;
        self.secret_bytes().to_vec().to_hex()
    }

    // TODO pub fn to_keypair
}

#[pyclass]
pub struct PublicKey {
    xonly_public_key: XOnlyPublicKey,
    source: String,
}

// START: TESTING
impl From<secp256k1::Error> for pyo3::PyErr {

}

// END TESTING

#[pymethods]
impl PublicKey {
    #[new]
    pub fn try_ney(key: &str) -> PyResult<Self> {
        match secp256k1::PublicKey::from_str(key) {
            Ok(public_key) => {
                let (xonly_public_key, _) = public_key.x_only_public_key();
                Ok(Self { xonly_public_key, source: (*key).to_string() })
            }
            Err(_e) => Ok(Self { 
                xonly_public_key: XOnlyPublicKey::from_str(key)?, 
                source: (*key).to_string() 
            }),
        }
    }
}
