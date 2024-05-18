use crate::derivation::gen1::WalletDerivationManager;
use crate::derivation::traits::WalletDerivationManagerTrait;
use crate::python::{
    publickey::PublicKey,
    xpub::XPub,
    xprv::XPrv,
};
use kaspa_addresses::Address;
use pyo3::prelude::*;


#[pyclass]
pub struct PublicKeyGenerator {
    hd_wallet: WalletDerivationManager,
}

#[pymethods]
impl PublicKeyGenerator {
    #[staticmethod]
    pub fn from_xpub(kpub: &str, cosigner_index: Option<u32>) -> PyResult<PublicKeyGenerator> {
        let kpub = XPub::try_new(kpub)?;
        let xpub = kpub.inner();
        let hd_wallet = WalletDerivationManager::from_extended_public_key(xpub.clone(), cosigner_index)?;
        Ok(Self { hd_wallet })
    }

    #[staticmethod]
    pub fn from_master_xprv(
        xprv: String,
        is_multisig: bool,
        account_index: u64,
        cosigner_index: Option<u32>,
    ) -> PyResult<PublicKeyGenerator> {
        let path = WalletDerivationManager::build_derivate_path(is_multisig, account_index, None, None)?;
        let xprv = XPrv::from_xprv_str(xprv)?.inner().clone().derive_path(&path)?;
        let xpub = xprv.public_key();
        let hd_wallet = WalletDerivationManager::from_extended_public_key(xpub, cosigner_index)?;
        Ok(Self { hd_wallet })
    }

    pub fn receive_pubkeys(&self, mut start: u32, mut end: u32) -> PyResult<Vec<PublicKey>> {
        if start > end {
            (start, end) = (end, start);
        }
        let pubkeys = self.hd_wallet.receive_pubkey_manager().derive_pubkey_range(start..end)?;        
        Ok(pubkeys.into_iter().map(|pk| PublicKey::from(pk)).collect())
    }

    pub fn receive_pubkey(&self, index: u32) -> PyResult<PublicKey> {
        Ok(self.hd_wallet.receive_pubkey_manager().derive_pubkey(index)?.into())
    }

    pub fn receive_pubkeys_as_strings(&self, mut start: u32, mut end: u32) -> PyResult<Vec<String>> {
        if start > end {
            (start, end) = (end, start);
        }
        let pubkeys = self.hd_wallet.receive_pubkey_manager().derive_pubkey_range(start..end)?;
        Ok(pubkeys.into_iter().map(|pk| PublicKey::from(pk).to_string()).collect())
    }

    pub fn receive_pubkey_as_string(&self, index: u32) -> PyResult<String> {
        Ok(self.hd_wallet.receive_pubkey_manager().derive_pubkey(index)?.to_string())
    }

    pub fn receive_addresses(&self, network_type: &str, mut start: u32, mut end: u32) -> PyResult<Vec<Address>> {
        if start > end {
            (start, end) = (end, start);
        }
        // let network_type = NetworkType::try_from(network_type)?;
        let pubkeys = self.hd_wallet.receive_pubkey_manager().derive_pubkey_range(start..end)?;
        let addresses = 
            pubkeys.into_iter().map(|pk| PublicKey::from(pk).to_address(network_type)).collect::<PyResult<Vec<Address>>>()?;
        Ok(addresses)
    }

    pub fn receive_address(&self, network_type: &str, index: u32) -> PyResult<Address> {
        PublicKey::from(self.hd_wallet.receive_pubkey_manager().derive_pubkey(index)?).to_address(network_type)
    }

    pub fn receive_addresses_as_strings(&self, network_type: &str, mut start: u32, mut end: u32) -> PyResult<Vec<String>> {
        if start > end {
            (start, end) = (end, start);
        }
        // let network_type = NetworkType::try_from(network_type)?;
        let pubkeys = self.hd_wallet.receive_pubkey_manager().derive_pubkey_range(start..end)?;
        let addresses = 
            pubkeys.into_iter().map(|pk| PublicKey::from(pk).to_address(network_type)).collect::<PyResult<Vec<Address>>>()?;
        Ok(addresses.into_iter().map(|a| a.address_to_string()).collect())
    }

    pub fn receive_address_as_string(&self, network_type: &str, index: u32) -> PyResult<String> {
        Ok(PublicKey::from(self.hd_wallet.receive_pubkey_manager().derive_pubkey(index)?)
            .to_address(network_type)?
            .to_string())
    }

    pub fn change_pubkeys(&self, mut start: u32, mut end: u32) -> PyResult<Vec<PublicKey>> {
        if start > end {
            (start, end) = (end, start);
        }
        let pubkeys = self.hd_wallet.receive_pubkey_manager().derive_pubkey_range(start..end)?;        
        Ok(pubkeys.into_iter().map(|pk| PublicKey::from(pk)).collect())
    }

    pub fn change_pubkey(&self, index: u32) -> PyResult<PublicKey> {
        Ok(self.hd_wallet.receive_pubkey_manager().derive_pubkey(index)?.into())
    }

    pub fn change_pubkeys_as_strings(&self, mut start: u32, mut end: u32) -> PyResult<Vec<String>> {
        if start > end {
            (start, end) = (end, start);
        }
        let pubkeys = self.hd_wallet.receive_pubkey_manager().derive_pubkey_range(start..end)?;
        Ok(pubkeys.into_iter().map(|pk| PublicKey::from(pk).to_string()).collect())
    }

    pub fn change_pubkey_as_string(&self, index: u32) -> PyResult<String> {
        Ok(self.hd_wallet.receive_pubkey_manager().derive_pubkey(index)?.to_string())
    }

    pub fn change_addresses(&self, network_type: &str, mut start: u32, mut end: u32) -> PyResult<Vec<Address>> {
        if start > end {
            (start, end) = (end, start);
        }
        // let network_type = NetworkType::try_from(network_type)?;
        let pubkeys = self.hd_wallet.receive_pubkey_manager().derive_pubkey_range(start..end)?;
        let addresses = 
            pubkeys.into_iter().map(|pk| PublicKey::from(pk).to_address(network_type)).collect::<PyResult<Vec<Address>>>()?;
        Ok(addresses)
    }

    pub fn change_address(&self, network_type: &str, index: u32) -> PyResult<Address> {
        PublicKey::from(self.hd_wallet.receive_pubkey_manager().derive_pubkey(index)?).to_address(network_type)
    }

    pub fn change_addresses_as_strings(&self, network_type: &str, mut start: u32, mut end: u32) -> PyResult<Vec<String>> {
        if start > end {
            (start, end) = (end, start);
        }
        // let network_type = NetworkType::try_from(network_type)?;
        let pubkeys = self.hd_wallet.receive_pubkey_manager().derive_pubkey_range(start..end)?;
        let addresses = 
            pubkeys.into_iter().map(|pk| PublicKey::from(pk).to_address(network_type)).collect::<PyResult<Vec<Address>>>()?;
        Ok(addresses.into_iter().map(|a| a.address_to_string()).collect())
    }

    pub fn change_address_as_string(&self, network_type: &str, index: u32) -> PyResult<String> {
        Ok(PublicKey::from(self.hd_wallet.receive_pubkey_manager().derive_pubkey(index)?)
            .to_address(network_type)?
            .to_string())
    }

    pub fn to_string(&self) -> PyResult<String> {
        Ok(self.hd_wallet.to_string(None).to_string())
    }
}