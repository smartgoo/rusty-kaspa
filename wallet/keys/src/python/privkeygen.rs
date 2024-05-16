use crate::derivation::gen1::WalletDerivationManager;
use crate::python::{
    privatekey::PrivateKey,
    xprv::XPrv
};
use kaspa_bip32::{ExtendedPrivateKey, ChildNumber, SecretKey};
use pyo3::prelude::*;

#[pyclass]
pub struct PrivateKeyGenerator {
    receive: ExtendedPrivateKey<SecretKey>,
    change: ExtendedPrivateKey<SecretKey>,
}

#[pymethods]
impl PrivateKeyGenerator {
    // TODO xprv param type
    #[new]
    pub fn new(xprv: String, is_multisig: bool, account_index: u64, cosigner_index: Option<u32>) -> PyResult<PrivateKeyGenerator> {
        let xprv = XPrv::from_xprv_str(xprv)?;
        let xprv = xprv.inner();
        let receive = xprv.clone().derive_path(&WalletDerivationManager::build_derivate_path(
            is_multisig,
            account_index,
            cosigner_index,
            Some(kaspa_bip32::AddressType::Receive)
        )?)?;
        let change = xprv.clone().derive_path(&WalletDerivationManager::build_derivate_path(
            is_multisig,
            account_index,
            cosigner_index,
            Some(kaspa_bip32::AddressType::Change)
        )?)?;

        Ok( Self { receive, change })
    }

    pub fn receive_key(&self, index: u32) -> PyResult<PrivateKey> {
        let xkey = self.change.derive_child(ChildNumber::new(index, false)?)?;
        Ok(PrivateKey::from(xkey.private_key()))
    }

    pub fn change_key(&self, index: u32) -> PyResult<PrivateKey> {
        let xkey = self.change.derive_child(ChildNumber::new(index, false)?)?;
        Ok(PrivateKey::from(xkey.private_key()))
    }
}