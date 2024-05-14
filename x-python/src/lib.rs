mod dbreader;

use pyo3::prelude::*;

use kaspa_addresses::Address;
use kaspa_bip32::mnemonic::phrase::Mnemonic;
use kaspa_wallet_keys::python::{
    derivation_path::DerivationPath,
    keypair::Keypair,
    privatekey::PrivateKey,
    publickey::PublicKey,
    publickey::XOnlyPublicKey,
    xprv::XPrv,
    xpub::XPub,
};
use kaspa_wallet_core::python::utils::{
    kaspa_to_sompi,
    sompi_to_kaspa,
    sompi_to_kaspa_string
};

#[pymodule]
fn kaspapy(m: &Bound<'_, PyModule>) -> PyResult<()> {

    // Classes - Addresses
    m.add_class::<Address>()?;
    m.add_class::<DerivationPath>()?;
    m.add_class::<Mnemonic>()?;
    m.add_class::<Keypair>()?;
    m.add_class::<PrivateKey>()?;
    m.add_class::<PublicKey>()?;
    m.add_class::<XOnlyPublicKey>()?;
    m.add_class::<XPrv>()?;
    m.add_class::<XPub>()?;

    // Classes - Database
    m.add_class::<dbreader::core::db_reader::DBReader>()?;

    // Functions
    m.add_function(wrap_pyfunction!(sompi_to_kaspa, m)?)?;
    m.add_function(wrap_pyfunction!(kaspa_to_sompi, m)?)?;
    m.add_function(wrap_pyfunction!(sompi_to_kaspa_string, m)?)?;

    Ok(())
}