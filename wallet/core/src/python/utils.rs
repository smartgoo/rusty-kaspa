use pyo3::prelude::*;

#[pyfunction]
pub fn sompi_to_kaspa(sompi: u64) -> PyResult<f64> {
    // let sompi = sompi.try_as_u64()?;
    Ok(crate::utils::sompi_to_kaspa(sompi))
}

#[pyfunction]
pub fn kaspa_to_sompi(kaspa: f64) -> u64 {
    crate::utils::kaspa_to_sompi(kaspa)
}

#[pyfunction]
pub fn sompi_to_kaspa_string(sompi: u64) -> PyResult<String> {
    // let sompi = sompi.try_as_u64()?;
    Ok(crate::utils::sompi_to_kaspa_string(sompi))
}

// #[pyfunction]
// pub fn sompi_to_kaspa_string_with_suffix(sompi: u64, wallet: &crate::wasm::wallet::Wallet) -> PyResult<String> {
//     let sompi = sompi.try_as_u64()?;
//     let network_type = wallet.wallet.network_id()?.network_type;
//     Ok(crate::utils::sompi_to_kaspa_string_with_suffix(sompi, &network_type))
// }
