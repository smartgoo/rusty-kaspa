use super::types::ToPy;
use kaspa_consensus_core::header::Header;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};

// TODO move to mod.rs
pub trait ToPyDict {
    fn to_py_dict<'a>(&'a self, py: Python<'a>) -> &'a PyDict;
}

// TODO move to it's own header.rs specific converter file
// Converts Header struct into PyO3 Python dict
// rusty-kaspa/consensus/core/src/header.rs
impl ToPyDict for Header {
    fn to_py_dict<'a>(&'a self, py: Python<'a>) -> &'a PyDict {
        let dict = PyDict::new(py);

        dict.set_item("hash", self.hash.to_py(py)).unwrap();
        dict.set_item("version", self.version).unwrap();
        dict.set_item("parents_by_level", self.parents_by_level.to_py(py)).unwrap();
        dict.set_item("hash_merkle_root", self.hash_merkle_root.to_py(py)).unwrap();
        dict.set_item("accepted_id_merkle_root", self.accepted_id_merkle_root.to_py(py)).unwrap();
        dict.set_item("utxo_commitment", self.utxo_commitment.to_py(py)).unwrap();
        dict.set_item("timestamp", self.timestamp).unwrap();
        dict.set_item("bits", self.bits).unwrap();
        dict.set_item("nonce", self.nonce).unwrap();
        dict.set_item("daa_score", self.daa_score).unwrap();
        // dict.set_item("blue_work", self.blue_work.to_py(py)).unwrap(); TODO
        dict.set_item("blue_score", self.blue_score).unwrap();
        dict.set_item("pruning_point", self.pruning_point.to_py(py)).unwrap();

        dict
    }
}
