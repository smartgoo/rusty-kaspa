use super::{types::ToPy, ToPyDict};
use kaspa_addresses::Prefix;
use kaspa_consensus_core::subnets::SubnetworkId;
use kaspa_consensus_core::tx::{Transaction, TransactionInput, TransactionOutput, TransactionOutpoint};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};
use std::fmt::Write;
use kaspa_txscript::extract_script_pub_key_address;


// Converts SubnetworkId to List[List[String]]
// ./rusty-kaspa/consensus/core/src/subnets.rs
impl ToPy for SubnetworkId {
    fn to_py(&self, py: Python) -> PyObject {        
        // Create a String to hold the formatted SubnetworkId.
        let mut hex_string = String::new();

        // Use the `write!` macro to write the formatted SubnetworkId to the String.
        write!(hex_string, "{}", self).expect("Formatting failed");

        // `hex_string` contains the SubnetworkId as a hexadecimal string.
        hex_string.into_py(py)
    }
}

// Converts TransactionInput struct into PyO3 Python dict
// ./rusty-kaspa/consensus/core/src/tx.rs
impl ToPyDict for TransactionOutpoint {
    fn to_py_dict<'a>(&'a self, py: Python<'a>) -> &'a PyDict {
        let dict = PyDict::new(py);
        dict.set_item("transaction_id", self.transaction_id.to_py(py)).unwrap();
        dict.set_item("index", self.index).unwrap();
        dict
    }
}

// Converts TransactionInput struct into PyO3 Python dict
// ./rusty-kaspa/consensus/core/src/tx.rs
impl ToPyDict for TransactionInput {
    fn to_py_dict<'a>(&'a self, py: Python<'a>) -> &'a PyDict {
        let dict = PyDict::new(py);
        dict.set_item("previous_outpoint", self.previous_outpoint.to_py_dict(py)).unwrap();
        // dict.set_item("signature_script", self.signature_script.to_py(py)).unwrap();
        dict.set_item("sequence", self.sequence).unwrap();
        dict.set_item("sig_op_count", self.sig_op_count).unwrap();
        dict
    }
}

// Converts TransactionOutput struct into PyO3 Python dict
// ./rusty-kaspa/consensus/core/src/tx.rs
impl ToPyDict for TransactionOutput {
    fn to_py_dict<'a>(&'a self, py: Python<'a>) -> &'a PyDict {
        let dict = PyDict::new(py);
        dict.set_item("value", self.value).unwrap();
        // dict.set_item("script_public_key", self.script_public_key.to_py_dict(py)).unwrap();
        dict.set_item("address", extract_script_pub_key_address(&self.script_public_key, Prefix::Mainnet).unwrap().to_string()).unwrap();
        dict
    }
}

// Converts Transaction struct into PyO3 Python dict
// ./rusty-kaspa/consensus/core/src/tx.rs
impl ToPyDict for Transaction {
    fn to_py_dict<'a>(&'a self, py: Python<'a>) -> &'a PyDict {
        let dict = PyDict::new(py);
        
        dict.set_item("version", self.version).unwrap();
        dict.set_item("inputs", self.inputs.iter().map(|input| input.to_py_dict(py)).collect::<Vec<_>>()).unwrap();
        dict.set_item("outputs", self.outputs.iter().map(|output| output.to_py_dict(py)).collect::<Vec<_>>()).unwrap();
        dict.set_item("lock_time", self.lock_time).unwrap();
        dict.set_item("subnetwork_id", self.subnetwork_id.to_py(py)).unwrap();
        dict.set_item("gas", self.gas).unwrap();

        let payload_bytes = PyBytes::new(py, &self.payload);
        dict.set_item("payload", payload_bytes).unwrap();

        dict.set_item("id", self.id().to_py(py)).unwrap();
        
        dict
    }
}

pub fn transactions_to_py_list(py: Python, transactions: Vec<Transaction>) -> Py<PyList> {
    let py_transactions = PyList::new(py, transactions.iter().map(|transaction| {

        // Convert each transaction to a Python dictionary
        transaction.to_py_dict(py).to_object(py)
    }));

    py_transactions.into()
}
