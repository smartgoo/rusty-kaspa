// use crate::{python::DerivationPath, ChildNumber, ExtendedPublicKey};
// use secp256k1::PublicKey;
// use std::str::FromStr;
// use pyo3::exceptions::PyException;
// use pyo3::prelude::*;

// #[pyclass]
// pub struct XPub {
//     inner: ExtendedPublicKey<PublicKey>,
// }

// #[pymethods]
// impl XPub {
//     #[new]
//     pub fn new(xpub: &str) -> PyResult<XPub> {
//         let inner = ExtendedPublicKey::<PublicKey>::from_str(xpub).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
//         Ok(Self { inner })
//     }

//     pub fn derive_child(&self, child_number: u32, hardened: Option<bool>) -> PyResult<XPub> {
//         let child_number = ChildNumber::new(child_number, hardened.unwrap_or(false)).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
//         let inner = self.inner.derive_child(child_number).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
//         Ok(Self { inner })
//     }

//     pub fn derive_path(&self, path: &str) -> PyResult<XPub> {
//         let path = DerivationPath::new(path)?;
//         let inner = self.inner.clone().derive_path(path.into()).map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;
//         Ok(Self { inner })
//     }

//     #[pyo3(name = "into_string")]
//     pub fn to_str(&self, prefix: &str) -> PyResult<String> {
//         Ok(self.inner.to_string(Some(prefix.try_into()
//             .map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)) 
//         )?)))
//     }
// }

// impl From<ExtendedPublicKey<PublicKey>> for XPub {
//     fn from(inner: ExtendedPublicKey<PublicKey>) -> Self {
//         Self { inner }
//     }
// }