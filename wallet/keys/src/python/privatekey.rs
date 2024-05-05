use crate::python::keypair::Keypair;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::str::FromStr;
use std::error::Error;

#[derive(Clone, Debug)]
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

impl From<&PrivateKey> for [u8; 32] {
    fn from(key: &PrivateKey) -> Self {
        key.secret_bytes()
    }
}

#[pymethods]
impl PrivateKey {
    #[new]
    // TODO return type Self or PrivateKey ?
    pub fn try_new(key: &str) -> PyResult<Self> {
        let secret_key = secp256k1::SecretKey::from_str(key)
            .map_err(|e| PyErr::new::<PyException, _>(format!("Failed to parse secret key: {}", e)))?;
        
        Ok(Self { inner: secret_key })
    }

    #[pyo3(name = "to_string")]
    pub fn to_hex(&self) -> String {
        use kaspa_utils::hex::ToHex;
        self.secret_bytes().to_vec().to_hex()
    }

    // TODO implement once Keypair is done
    pub fn to_keypair(&self) -> PyResult<Keypair> {
        Keypair::from_private_key(self)
    }

    // TODO implement once PublicKey is done
    // pub fn to_public_key() -> PyResult<PublicKey> {}

    // TODO implement once Address is done
    // pub fn to_address() -> PyResult<Address> {}

    // TODO implement once Address is done
    // pub fn to_address_ecdsa() -> PyResult<Address> {}
}

// TODO
// impl PrivateKey {
//     pub fn try_from_slice(data: &[u8]) -> Result<PrivateKey> {
//         Ok(Self { inner: secp256k1::SecretKey::from_slice(data)? })
//     }
// }