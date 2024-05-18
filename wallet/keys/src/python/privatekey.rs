use crate::python::{
    keypair::Keypair,
    publickey::PublicKey,
};
pub use kaspa_addresses::{Address, Version as AddressVersion};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::str::FromStr;

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
            .map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?; // TODO get rid of map_err
        
        Ok(Self { inner: secret_key })
    }

    #[pyo3(name = "to_string")]
    pub fn to_hex(&self) -> String {
        use kaspa_utils::hex::ToHex;
        self.secret_bytes().to_vec().to_hex()
    }

    pub fn to_keypair(&self) -> PyResult<Keypair> {
        Keypair::from_private_key(self)
    }

    pub fn to_public_key(&self) -> PyResult<PublicKey> {
        Ok(PublicKey::from(secp256k1::PublicKey::from_secret_key_global(&self.inner)))
    }

    pub fn to_address(&self, network: &str) -> PyResult<Address> {
        let public_key = secp256k1::PublicKey::from_secret_key_global(&self.inner);
        let (x_only_public_key, _) = public_key.x_only_public_key();
        let payload = x_only_public_key.serialize();
        let address = Address::new(network.try_into()?, AddressVersion::PubKey, &payload);
        Ok(address)
    }

    pub fn to_address_ecdsa(&self, network: &str) -> PyResult<Address> {
        let public_key = secp256k1::PublicKey::from_secret_key_global(&self.inner);
        let payload = public_key.serialize();
        let address = Address::new(network.try_into()?, AddressVersion::PubKeyECDSA, &payload);
        Ok(address)
    }
}