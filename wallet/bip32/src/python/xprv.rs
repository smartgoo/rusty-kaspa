// use crate::{
//     python::{DerivationPath, XPub},
//     ChildNumber, Error, ExtendedPrivateKey, Result, SecretKey,
// };
// use kaspa_utils::hex::*;
// use std::str::FromStr;
// use pyo3::exceptions::PyException;
// use pyo3::prelude::*;

// #[pyclass]
// pub struct XPrv {
//     inner: ExtendedPrivateKey<SecretKey>,
// }

// #[pymethods]
// impl XPrv {
//     #[new]
//     // TODO return self or XPrv? Static method if XPrv
//     pub fn new(seed: String) -> PyResult<XPrv> { 
//         let seed_bytes = Vec::<u8>::from_hex(&seed).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;

//         let inner = ExtendedPrivateKey::<SecretKey>::new(seed_bytes).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
//         Ok(Self { inner })
//     }

//     // TODO static method based on return type?
//     pub fn derive_child(&self, child_number: u32, hardened: Option<bool>) -> PyResult<XPrv> {
//         let child_number = ChildNumber::new(child_number, hardened.unwrap_or(false)).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
//         let inner = self.inner.derive_child(child_number).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
//         Ok(Self { inner })
//     }

//     // TODO is path type correct?
//     pub fn derive_path(&self, path: &str) -> PyResult<XPrv> {
//         let path = DerivationPath::new(path)?; // TODO is this correct?
//         let inner = self.inner.clone().derive_path(path.into()).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
//         Ok(Self { inner })
//     }

//     #[pyo3(name = "into_string")]
//     pub fn to_str(&self, prefix: &str) -> PyResult<String> {
//         let str = self.inner.to_extended_key(prefix.try_into().map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?);
//         Ok(str.to_string())
//     }

//     pub fn public_key(&self) -> PyResult<XPub> {
//         let public_key = self.inner.public_key();
//         Ok(public_key.into())
//     }
// }