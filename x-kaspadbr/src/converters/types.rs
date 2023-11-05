use kaspa_hashes::Hash;
use kaspa_math::Uint192;
use pyo3::prelude::*;

pub trait ToPy {
    fn to_py(&self, py: Python) -> PyObject;
}

// Conversions for custom types

// Converts Hash to String
// ./rusty-kaspa/crypto/hashes/src/lib.rs
impl ToPy for Hash {
    fn to_py(&self, py: Python) -> PyObject {
        self.to_string().into_py(py)
    }
}

// Converts <Vec<Vec<Hash>> to List[List[String]]
// ./rusty-kaspa/crypto/hashes/src/lib.rs
impl ToPy for Vec<Vec<Hash>> {
    fn to_py(&self, py: Python) -> PyObject {
        let outer_list: Vec<_> = self
            .iter()
            .map(|inner_vec| {
                let inner_list: Vec<_> = inner_vec.iter().map(|hash| hash.to_py(py)).collect();
                inner_list.into_py(py)
            })
            .collect();
        outer_list.into_py(py)
    }
}

// Converts Uint192 to String
// Uint192 defined in: ./rusty-kaspa/crypto/hashes/src/lib.rs
impl ToPy for Uint192 {
    // TODO convert to int
    // Using string temporarily because concerned about precision after conversion
    fn to_py(&self, py: Python) -> PyObject {
        format!("{:x}", self).into_py(py)
    }
}

// HashSet type converter
