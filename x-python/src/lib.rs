mod dbreader;

use pyo3::prelude::*;

use kaspa_addresses::Address;
use kaspa_bip32::mnemonic::phrase::Mnemonic;
use kaspa_consensus_python::keypair::{Keypair, PrivateKey, PublicKey};
use kaspa_wallet_core::python::utils::{
    kaspa_to_sompi,
    sompi_to_kaspa,
    sompi_to_kaspa_string
};

#[pymodule]
fn kaspapy(m: &Bound<'_, PyModule>) -> PyResult<()> {

    // Classess
    m.add_class::<Address>()?;
    m.add_class::<Mnemonic>()?;
    m.add_class::<Keypair>()?;
    m.add_class::<PrivateKey>()?;
    m.add_class::<PublicKey>()?;
    m.add_class::<dbreader::core::db_reader::DBReader>()?;

    // Functions
    m.add_function(wrap_pyfunction!(sompi_to_kaspa, m)?)?;
    m.add_function(wrap_pyfunction!(kaspa_to_sompi, m)?)?;
    m.add_function(wrap_pyfunction!(sompi_to_kaspa_string, m)?)?;

    Ok(())
}