pub use kaspa_addresses::{Address, Version as AddressVersion};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::str::FromStr;

#[derive(Clone, Debug)]
#[pyclass]
pub struct PublicKey {
    pub xonly_public_key: secp256k1::XOnlyPublicKey,
    pub public_key: Option<secp256k1::PublicKey>,
}

#[pymethods]
impl PublicKey {
    #[new]
    // TODO return type of Self or PublicKey ?
    pub fn try_new(key: &str) -> PyResult<Self> {
        match secp256k1::PublicKey::from_str(key) {
            Ok(public_key) => Ok((&public_key).into()),
            Err(_e) => Ok(Self { 
                xonly_public_key: secp256k1::XOnlyPublicKey::from_str(key)
                    .map_err(|err| PyErr::new::<PyException, _>(format!("{}", err)))?, // TODO get rid of map_err
                public_key: None 
            }),
        }
    }

    #[pyo3(name = "to_string")]
    pub fn to_string_impl(&self) -> String {
        self.public_key.as_ref().map(|pk| pk.to_string()).unwrap_or_else(|| self.xonly_public_key.to_string())
    }  

    // TODO param network should be of type Network
    pub fn to_address(&self, network: &str) -> PyResult<Address> {
        let payload = &self.xonly_public_key.serialize();
        let address = Address::new(
            network.try_into()?, 
            AddressVersion::PubKey, payload
        );
        Ok(address)
    }

    // TODO param network should be of type Network
    pub fn to_address_ecdsa(&self, network: &str) -> PyResult<Address> {
        let payload = &self.xonly_public_key.serialize();
        let address = Address::new(
            network.try_into()?,
            AddressVersion::PubKeyECDSA, payload
        );
        Ok(address)
    }

    // TODO once XOnlyPublicKey implemented
    // pub fn to_x_only_public_key(&self) -> XOnlyPublicKey {}
}

impl From<&secp256k1::PublicKey> for PublicKey {
    fn from(public_key: &secp256k1::PublicKey) -> Self {
        let (xonly_public_key, _) = public_key.x_only_public_key();
        Self { xonly_public_key, public_key: Some(*public_key) }
    }
}

impl From<secp256k1::PublicKey> for PublicKey {
    fn from(public_key: secp256k1::PublicKey) -> Self {
        let (xonly_public_key, _) = public_key.x_only_public_key();
        Self { xonly_public_key, public_key: Some(public_key) }
    }
}

// TODO XOnlyPublicKey
#[pyclass]
pub struct XOnlyPublicKey {
    pub inner: secp256k1::XOnlyPublicKey,
}

impl XOnlyPublicKey {
    pub fn new(inner: secp256k1::XOnlyPublicKey) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl XOnlyPublicKey {
    #[new]
    pub fn try_new(key: &str) -> PyResult<XOnlyPublicKey> {
        let x_only_public_key = secp256k1::XOnlyPublicKey::from_str(key).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
        Ok(x_only_public_key.into())
    }

    #[pyo3(name = "to_string")]
    pub fn to_string_impl(&self) -> String {
        self.inner.to_string()
    }

    pub fn to_address(&self, network: &str) -> PyResult<Address> {
        let payload = &self.inner.serialize();
        let address = Address::new(network.try_into()?, AddressVersion::PubKey, payload);
        Ok(address)
    }

    pub fn to_address_ecdsa(&self, network: &str) -> PyResult<Address> {
        let payload = &self.inner.serialize();
        let address = Address::new(network.try_into()?, AddressVersion::PubKeyECDSA, payload);
        Ok(address)
    }

    #[staticmethod]
    pub fn from_address(address: &Address) -> PyResult<XOnlyPublicKey> {
        let x_only_public_key = secp256k1::XOnlyPublicKey::from_slice(&address.payload).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
        Ok(x_only_public_key.into())
    }
}

impl From<secp256k1::XOnlyPublicKey> for XOnlyPublicKey {
    fn from(inner: secp256k1::XOnlyPublicKey) -> Self {
        Self { inner }
    }
}