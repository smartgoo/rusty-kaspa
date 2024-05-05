pub use crate::python::privatekey::PrivateKey;
pub use crate::python::publickey::PublicKey;
use kaspa_addresses::{Address, Version as AddressVersion};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use secp256k1::{Secp256k1, XOnlyPublicKey};

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
        let payload = &self.xonly_public_key.serialize();
        let address = Address::new(
            network.try_into().map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?,
            AddressVersion::PubKey, payload
        );
        Ok(address)
    }

    // TODO network param should be of type Network
    pub fn to_address_ecdsa(&self, network: &str) -> PyResult<Address> {
        let payload = &self.public_key.serialize();
        let address = Address::new(
            network.try_into().map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?,
            AddressVersion::PubKeyECDSA, payload
        );
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
    pub fn from_private_key(secret_key: &PrivateKey) -> PyResult<Keypair> {
        let secp = Secp256k1::new();
        let secret_key = secp256k1::SecretKey::from_slice(&secret_key.secret_bytes())
            .map_err(|e| { PyErr::new::<PyException, _>(format!("{e}"))} )?;
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
        let (xonly_public_key, _) = public_key.x_only_public_key();
        Ok(Keypair::new(secret_key, public_key, xonly_public_key))
    }
}