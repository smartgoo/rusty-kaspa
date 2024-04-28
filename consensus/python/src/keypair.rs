// consensus/wasm/src/keypair.rs

use kaspa_addresses::{Address, Version as AddressVersion};
use kaspa_consensus_core::network::wasm::Network;
use secp256k1::{Secp256k1, XOnlyPublicKey};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass]
pub struct Keypair {
    secret_key: secp256k1::SecretKey,
    public_key: secp256k1::PublicKey,
    xonly_public_key: XOnlyPublicKey
}

impl Keypair {
    fn new(secret_key: secp256k1::SecretKey, public_key: secp256k1::PublicKey, xonly_public_key: XOnlyPublicKey) -> Self {
        Self { secret_key, public_key, xonly_public_key }
    }
}

#[pymethods]
impl Keypair {
    #[getter]
    #[pyo3(name = "public_key")]
    pub fn get_public_key(&self) -> String {
        self.public_key.to_string()
    }

    #[getter]
    #[pyo3(name = "private_key")]
    pub fn get_private_key(&self) -> PrivateKey {
        (&self.secret_key).into()
    }

    #[getter]
    #[pyo3(name="xonly_public_key")]
    pub fn get_xonly_public_key(&self) -> String {
        self.xonly_public_key.to_string()
    }

    // TODO network param should be of type Network
    pub fn to_address(&self, network: &str) -> PyResult<Address> {
        let pk = PublicKey { xonly_public_key: self.xonly_public_key, source: self.public_key.to_string() };
        let address = pk.to_address(network)?;
        Ok(address)
    }

    // TODO network param should be of type Network
    pub fn to_address_ecdsa(&self, network: &str) -> PyResult<Address> {
        let pk = PublicKey { xonly_public_key: self.xonly_public_key, source: self.public_key.to_string() };
        let address = pk.to_address_ecdsa(network)?;
        Ok(address)
    }

    #[staticmethod]
    pub fn random() -> PyResult<Keypair> {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        let (xonly_public_key, _) = public_key.x_only_public_key();
        Ok(Keypair::new(secret_key, public_key, xonly_public_key))
    }

    // TODO secret_key param should be type &PrivateKey
    #[staticmethod]
    pub fn from_private_key(secret_key: &str) -> PyResult<Keypair> {
        let secret_key = PrivateKey::try_new(secret_key)?;

        let secp = Secp256k1::new();
        let secret_key = secp256k1::SecretKey::from_slice(&secret_key.secret_bytes())
            .map_err(|e| { PyErr::new::<PyException, _>(format!("{e}"))} )?;
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
        let (xonly_public_key, _) = public_key.x_only_public_key();
        Ok(Keypair::new(secret_key, public_key, xonly_public_key))
    }
}

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
    // TODO return type Self or PviateKey ?
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

    pub fn to_keypair(&self) -> PyResult<Keypair> {
        Keypair::from_private_key(&self.to_hex())
    }
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct PublicKey {
    xonly_public_key: XOnlyPublicKey,
    source: String,
}

#[pymethods]
impl PublicKey {
    #[new]
    // TODO return type of Self or PublicKey ?
    pub fn try_new(key: &str) -> PyResult<Self> {
        match secp256k1::PublicKey::from_str(key) {
            Ok(public_key) => {
                let (xonly_public_key, _) = public_key.x_only_public_key();
                Ok(Self { xonly_public_key, source: (*key).to_string() })
            }
            Err(_e) => Ok(
                Self { 
                    xonly_public_key: XOnlyPublicKey::from_str(key)
                        .map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?, 
                    source: (*key).to_string() 
                }
            ),
        }
    }

    #[pyo3(name = "to_string")]
    pub fn py_to_string(&self) -> String {
        self.source.clone()
    }  

    // TODO param network should be of type Network
    pub fn to_address(&self, network: &str) -> PyResult<Address> {
        let payload = &self.xonly_public_key.serialize();
        let address = Address::new(
            network.try_into().map_err(|err| PyErr::new::<PyException, _>(format!("{}", err)))?, 
            AddressVersion::PubKey, payload
        );
        Ok(address)
    }

    // TODO param network should be of type Network
    pub fn to_address_ecdsa(&self, network: &str) -> PyResult<Address> {
        let payload = &self.xonly_public_key.serialize();
        let address = Address::new(
            network.try_into().map_err(|err| PyErr::new::<PyException, _>(format!("{}", err)))?,
            AddressVersion::PubKeyECDSA, payload
        );
        Ok(address)
    }
}
