use crate::error::Error;
use kaspa_addresses::{Address, Version as AddressVersion};
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
    pub fn to_x_only_public_key(&self) -> XOnlyPublicKey {
        self.xonly_public_key.into()
    }
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

impl std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_impl())
    }
}

impl From<PublicKey> for secp256k1::XOnlyPublicKey {
    fn from(value: PublicKey) -> Self {
        value.xonly_public_key
    }
}

impl TryFrom<PublicKey> for secp256k1::PublicKey {
    type Error = Error;
    fn try_from(value: PublicKey) -> Result<Self, Self::Error> {
        value.public_key.ok_or(Error::InvalidPublicKey)
    }
}

// impl TryFrom<Vec<PublicKey>> for Vec<secp256k1::PublicKey> {
//     type Error = Error;
//     fn try_from(value: Vec<PublicKey>) -> Result<Self, Self::Error> {
//         // Convert each KaspaPublicKey to Secp256k1PublicKey
//         value.into_iter().map(|pk| pk.try_into()).collect()
//     }
// }

pub trait ToSecp256k1Vec {
    type Error;

    fn to_secp256k1_vec(self) -> Result<Vec<secp256k1::PublicKey>, Self::Error>;
}

impl ToSecp256k1Vec for Vec<PublicKey> {
    type Error = Error;

    fn to_secp256k1_vec(self) -> Result<Vec<secp256k1::PublicKey>, Self::Error> {
        self.into_iter().map(|pk| pk.try_into()).collect()
    }
}

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