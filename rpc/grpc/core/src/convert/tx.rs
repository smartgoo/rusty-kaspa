use crate::protowire::{self, RpcTransactionVerboseDataVerbosity};
use crate::{from, try_from};
use kaspa_rpc_core::{FromRpcHex, RpcError, RpcHash, RpcResult, RpcScriptVec, ToRpcHex};
use std::str::FromStr;

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

from!(item: &kaspa_rpc_core::RpcTransaction, protowire::RpcTransaction, {
    Self {
        version: item.version.into(),
        inputs: item.inputs.iter().map(protowire::RpcTransactionInput::from).collect(),
        outputs: item.outputs.iter().map(protowire::RpcTransactionOutput::from).collect(),
        lock_time: item.lock_time,
        subnetwork_id: item.subnetwork_id.to_string(),
        gas: item.gas,
        payload: item.payload.to_rpc_hex(),
        mass: item.mass,
        verbose_data: item.verbose_data.as_ref().map(|x| x.into()),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionVerbosity, protowire::RpcTransactionVerbosity, {
    Self {
        include_version: item.include_version,
        input_verbosity: item.input_verbosity.as_ref().map(protowire::RpcTransactionInputVerbosity::from),
        output_verbosity: item.output_verbosity.as_ref().map(protowire::RpcTransactionOutputVerbosity::from),
        include_lock_time: item.include_lock_time,
        include_subnetwork_id: item.include_subnetwork_id,
        include_gas: item.include_gas,
        include_payload: item.include_payload,
        include_mass: item.include_mass,
        verbose_data_verbosity: item.verbose_data_verbosity.as_ref().map(RpcTransactionVerboseDataVerbosity::from),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionInput, protowire::RpcTransactionInput, {
    Self {
        previous_outpoint: item.previous_outpoint.as_ref().map(protowire::RpcOutpoint::from),
        signature_script: item.signature_script.to_rpc_hex(),
        sequence: item.sequence,
        sig_op_count: item.sig_op_count.into(),
        verbose_data: item.verbose_data.as_ref().map(protowire::RpcTransactionInputVerboseData::from),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionInputVerbosity, protowire::RpcTransactionInputVerbosity, {
    Self {
        include_previous_outpoint: item.include_previous_outpoint,
        include_signature_script: item.include_signature_script,
        include_sequence: item.include_sequence,
        include_sig_op_count: item.include_sig_op_count,
        verbose_data_verbosity: item.verbose_data_verbosity.as_ref().map(protowire::RpcTransactionInputVerboseDataVerbosity::from),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionOutput, protowire::RpcTransactionOutput, {
    Self {
        amount: item.value,
        script_public_key: Some((&item.script_public_key).into()),
        verbose_data: item.verbose_data.as_ref().map(|x| x.into()),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionOutputVerbosity, protowire::RpcTransactionOutputVerbosity, {
    Self {
        include_amount: item.include_amount,
        include_script_public_key: item.include_script_public_key,
        verbose_data_verbosity: item.verbose_data_verbosity.as_ref().map(protowire::RpcTransactionOutputVerboseDataVerbosity::from),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionOutpoint, protowire::RpcOutpoint, {
    Self { transaction_id: item.transaction_id.to_string(), index: item.index }
});

from!(item: &kaspa_rpc_core::RpcTransactionOutpointVerbosity, protowire::RpcOutpointVerbosity, {
    Self {
        include_transaction_id: item.include_transaction_id,
        include_index: item.include_index,
    }
});

from!(item: &kaspa_rpc_core::RpcUtxoEntry, protowire::RpcUtxoEntry, {
    Self {
        amount: item.amount,
        script_public_key: Some((&item.script_public_key).into()),
        block_daa_score: item.block_daa_score,
        is_coinbase: item.is_coinbase,
    }
});

from!(item: &kaspa_rpc_core::RpcUtxoEntryVerbosity, protowire::RpcUtxoEntryVerbosity, {
    Self {
        include_amount: item.include_amount,
        include_script_public_key: item.include_script_public_key,
        include_block_daa_score: item.include_block_daa_score,
        include_is_coinbase: item.include_is_coinbase,
    }
});

from!(item: &kaspa_rpc_core::RpcScriptPublicKey, protowire::RpcScriptPublicKey, {
    Self { version: item.version().into(), script_public_key: item.script().to_rpc_hex() }
});

from!(item: &kaspa_rpc_core::RpcTransactionVerboseData, protowire::RpcTransactionVerboseData, {
    Self {
        transaction_id: item.transaction_id.to_string(),
        hash: item.hash.to_string(),
        compute_mass: item.compute_mass,
        block_hash: item.block_hash.to_string(),
        block_time: item.block_time,
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionVerboseDataVerbosity, protowire::RpcTransactionVerboseDataVerbosity, {
    Self {
        include_transaction_id: item.include_transaction_id,
        include_hash: item.include_hash,
        include_compute_mass: item.include_compute_mass,
        include_block_hash: item.include_block_hash,
        include_block_time: item.include_block_time,
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionInputVerboseData, protowire::RpcTransactionInputVerboseData, {
    Self {
        utxo_entry: item.utxo_entry.as_ref().map(|x| x.into()),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionInputVerboseDataVerbosity, protowire::RpcTransactionInputVerboseDataVerbosity, {
    Self {
        utxo_entry_verbosity: item.utxo_entry_verbosity.as_ref().map(protowire::RpcUtxoEntryVerbosity::from),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionOutputVerboseData, protowire::RpcTransactionOutputVerboseData, {
    Self {
        script_public_key_type: item.script_public_key_type.to_string(),
        script_public_key_address: if let Some(address) = item.script_public_key_address.as_ref() {
            address.into()
        } else {
            Default::default()
        },
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionOutputVerboseDataVerbosity, protowire::RpcTransactionOutputVerboseDataVerbosity, {
    Self {
        include_script_public_key_type: item.include_script_public_key_type,
        include_script_public_key_address: item.include_script_public_key_address,
    }
});

from!(item: &kaspa_rpc_core::RpcAcceptedTransactionIds, protowire::RpcAcceptedTransactionIds, {
    Self {
        accepting_block_hash: item.accepting_block_hash.to_string(),
        accepted_transaction_ids: item.accepted_transaction_ids.iter().map(|x| x.to_string()).collect(),
    }
});

from!(item: &kaspa_rpc_core::RpcUtxosByAddressesEntry, protowire::RpcUtxosByAddressesEntry, {
    Self {
        address: item.address.as_ref().map_or("".to_string(), |x| x.into()),
        outpoint: Some((&item.outpoint).into()),
        utxo_entry: Some((&item.utxo_entry).into()),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionAcceptingBlockLocator, protowire::RpcTransactionAcceptingBlockLocator, {
    Self {
        accepting_chain_block: item.accepting_chain_block.to_string(),
        transaction_ids: item.transaction_ids.iter().map(|x| x.to_string()).collect(),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionAcceptingDaaScoreLocator, protowire::RpcTransactionAcceptingDaaScoreLocator, {
    Self {
        accepting_daa_score: item.accepting_daa_score,
        transaction_ids: item.transaction_ids.iter().map(|x| x.to_string()).collect(),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionInclusionIndicesLocator, protowire::RpcTransactionInclusionIndicesLocator, {
    Self {
        block_hash: item.block_hash.to_string(),
        indices_within_block: item.indices_within_block.to_owned(),
    }
});

from!(item: &kaspa_rpc_core::RpcTransactionLocator, protowire::RpcTransactionLocator, {
    match item {
        kaspa_rpc_core::RpcTransactionLocator::ByAcceptingBlock(ref transaction_locator) => {
            Self { transaction_locator: Some(protowire::rpc_transaction_locator::TransactionLocator::ByAcceptingBlock(transaction_locator.into())) }
        }
        kaspa_rpc_core::RpcTransactionLocator::ByAcceptingDaaScore(ref transaction_locator) => {
            Self { transaction_locator: Some(protowire::rpc_transaction_locator::TransactionLocator::ByAcceptingDaaScore(transaction_locator.into())) }
        }
        kaspa_rpc_core::RpcTransactionLocator::ByInclusionIndices(ref transaction_locator) => {
            Self { transaction_locator: Some(protowire::rpc_transaction_locator::TransactionLocator::ByInclusionIndices(transaction_locator.into())) }
        }
    }
});

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

try_from!(item: &protowire::RpcTransaction, kaspa_rpc_core::RpcTransaction, {
    Self {
        version: item.version.try_into()?,
        inputs: item
            .inputs
            .iter()
            .map(kaspa_rpc_core::RpcTransactionInput::try_from)
            .collect::<RpcResult<Vec<kaspa_rpc_core::RpcTransactionInput>>>()?,
        outputs: item
            .outputs
            .iter()
            .map(kaspa_rpc_core::RpcTransactionOutput::try_from)
            .collect::<RpcResult<Vec<kaspa_rpc_core::RpcTransactionOutput>>>()?,
        lock_time: item.lock_time,
        subnetwork_id: kaspa_rpc_core::RpcSubnetworkId::from_str(&item.subnetwork_id)?,
        gas: item.gas,
        payload: Vec::from_rpc_hex(&item.payload)?,
        mass: item.mass,
        verbose_data: item.verbose_data.as_ref().map(kaspa_rpc_core::RpcTransactionVerboseData::try_from).transpose()?,
    }
});

try_from!(item: &protowire::RpcTransactionVerbosity, kaspa_rpc_core::RpcTransactionVerbosity, {
    Self {
        include_version: item.include_version,
        input_verbosity: item.input_verbosity.as_ref().map(kaspa_rpc_core::RpcTransactionInputVerbosity::try_from).transpose()?,
        output_verbosity: item.output_verbosity.as_ref().map(kaspa_rpc_core::RpcTransactionOutputVerbosity::try_from).transpose()?,
        include_lock_time: item.include_lock_time,
        include_subnetwork_id: item.include_subnetwork_id,
        include_gas: item.include_gas,
        include_payload: item.include_payload,
        include_mass: item.include_mass,
        verbose_data_verbosity: item.verbose_data_verbosity.as_ref().map(kaspa_rpc_core::RpcTransactionVerboseDataVerbosity::try_from).transpose()?,
    }
});

try_from!(item: &protowire::RpcTransactionInput, kaspa_rpc_core::RpcTransactionInput, {
    Self {
        previous_outpoint: item
            .previous_outpoint
            .as_ref()
            .map(kaspa_rpc_core::RpcTransactionOutpoint::try_from)
            .transpose()?,
        signature_script: Vec::from_rpc_hex(&item.signature_script)?,
        sequence: item.sequence,
        sig_op_count: item.sig_op_count.try_into()?,
        verbose_data: item.verbose_data.as_ref().map(kaspa_rpc_core::RpcTransactionInputVerboseData::try_from).transpose()?,
    }
});

try_from!(item: &protowire::RpcTransactionInputVerbosity, kaspa_rpc_core::RpcTransactionInputVerbosity, {
    Self {
        include_previous_outpoint: item.include_previous_outpoint,
        include_signature_script: item.include_signature_script,
        include_sequence: item.include_sequence,
        include_sig_op_count: item.include_sig_op_count,
        verbose_data_verbosity: item.verbose_data_verbosity.as_ref().map(kaspa_rpc_core::RpcTransactionInputVerboseDataVerbosity::try_from).transpose()?,
    }
});

try_from!(item: &protowire::RpcTransactionOutput, kaspa_rpc_core::RpcTransactionOutput, {
    Self {
        value: item.amount,
        script_public_key: item
            .script_public_key
            .as_ref()
            .ok_or_else(|| RpcError::MissingRpcFieldError("RpcTransactionOutput".to_string(), "script_public_key".to_string()))?
            .try_into()?,
        verbose_data: item.verbose_data.as_ref().map(kaspa_rpc_core::RpcTransactionOutputVerboseData::try_from).transpose()?,
    }
});

try_from!(item: &protowire::RpcTransactionOutputVerbosity, kaspa_rpc_core::RpcTransactionOutputVerbosity, {
    Self {
        include_amount: item.include_amount,
        include_script_public_key: item.include_script_public_key,
        verbose_data_verbosity: item.verbose_data_verbosity.as_ref().map(kaspa_rpc_core::RpcTransactionOutputVerboseDataVerbosity::try_from).transpose()?,
    }
});

try_from!(item: &protowire::RpcOutpoint, kaspa_rpc_core::RpcTransactionOutpoint, {
    Self { transaction_id: RpcHash::from_str(&item.transaction_id)?, index: item.index }
});

try_from!(item: &protowire::RpcOutpointVerbosity, kaspa_rpc_core::RpcTransactionOutpointVerbosity, {
    Self {
        include_transaction_id: item.include_transaction_id,
        include_index: item.include_index,
    }
});

try_from!(item: &protowire::RpcUtxoEntry, kaspa_rpc_core::RpcUtxoEntry, {
    Self {
        amount: item.amount,
        script_public_key: item
            .script_public_key
            .as_ref()
            .ok_or_else(|| RpcError::MissingRpcFieldError("RpcTransactionOutput".to_string(), "script_public_key".to_string()))?
            .try_into()?,
        block_daa_score: item.block_daa_score,
        is_coinbase: item.is_coinbase,
    }
});

try_from!(item: &protowire::RpcUtxoEntryVerbosity, kaspa_rpc_core::RpcUtxoEntryVerbosity, {
    Self {
        include_amount: item.include_amount,
        include_script_public_key: item.include_script_public_key,
        include_block_daa_score: item.include_block_daa_score,
        include_is_coinbase: item.include_is_coinbase,
    }
});

try_from!(item: &protowire::RpcScriptPublicKey, kaspa_rpc_core::RpcScriptPublicKey, {
    Self::new(u16::try_from(item.version)?, RpcScriptVec::from_rpc_hex(item.script_public_key.as_str())?)
});

try_from!(item: &protowire::RpcTransactionVerboseData, kaspa_rpc_core::RpcTransactionVerboseData, {
    Self {
        transaction_id: RpcHash::from_str(&item.transaction_id)?,
        hash: RpcHash::from_str(&item.hash)?,
        compute_mass: item.compute_mass,
        block_hash: RpcHash::from_str(&item.block_hash)?,
        block_time: item.block_time,
    }
});

try_from!(item: &protowire::RpcTransactionVerboseDataVerbosity, kaspa_rpc_core::RpcTransactionVerboseDataVerbosity, {
    Self {
        include_transaction_id: item.include_transaction_id,
        include_hash: item.include_hash,
        include_compute_mass: item.include_compute_mass,
        include_block_hash: item.include_block_hash,
        include_block_time: item.include_block_time,
    }
});

try_from!(item: &protowire::RpcTransactionInputVerboseData, kaspa_rpc_core::RpcTransactionInputVerboseData, {
    Self {
        utxo_entry: item.utxo_entry.as_ref().map(kaspa_rpc_core::RpcUtxoEntry::try_from).transpose()?,
    }
});

try_from!(item: &protowire::RpcTransactionInputVerboseDataVerbosity, kaspa_rpc_core::RpcTransactionInputVerboseDataVerbosity, {
    Self {
        utxo_entry_verbosity: item.utxo_entry_verbosity.as_ref().map(kaspa_rpc_core::RpcUtxoEntryVerbosity::try_from).transpose()?,
    }
});

try_from!(item: &protowire::RpcTransactionOutputVerboseData, kaspa_rpc_core::RpcTransactionOutputVerboseData, {
    Self {
        script_public_key_type: item.script_public_key_type.as_str().try_into()?,
        script_public_key_address: if !item.script_public_key_address.is_empty() {
            Some(kaspa_rpc_core::RpcAddress::try_from(item.script_public_key_address.as_ref())?)
        } else {
            None
        },
    }
});

try_from!(item: &protowire::RpcTransactionOutputVerboseDataVerbosity, kaspa_rpc_core::RpcTransactionOutputVerboseDataVerbosity, {
    Self {
        include_script_public_key_type: item.include_script_public_key_type,
        include_script_public_key_address: item.include_script_public_key_address,
    }
});

try_from!(item: &protowire::RpcAcceptedTransactionIds, kaspa_rpc_core::RpcAcceptedTransactionIds, {
    Self {
        accepting_block_hash: RpcHash::from_str(&item.accepting_block_hash)?,
        accepted_transaction_ids: item.accepted_transaction_ids.iter().map(|x| RpcHash::from_str(x)).collect::<Result<Vec<_>, _>>()?,
    }
});

try_from!(item: &protowire::RpcUtxosByAddressesEntry, kaspa_rpc_core::RpcUtxosByAddressesEntry, {
    let address = if item.address.is_empty() { None } else { Some(item.address.as_str().try_into()?) };
    Self {
        address,
        outpoint: item
            .outpoint
            .as_ref()
            .ok_or_else(|| RpcError::MissingRpcFieldError("UtxosByAddressesEntry".to_string(), "outpoint".to_string()))?
            .try_into()?,
        utxo_entry: item
            .utxo_entry
            .as_ref()
            .ok_or_else(|| RpcError::MissingRpcFieldError("UtxosByAddressesEntry".to_string(), "utxo_entry".to_string()))?
            .try_into()?,
    }
});

try_from!(item: &protowire::RpcTransactionAcceptingBlockLocator, kaspa_rpc_core::RpcTransactionAcceptingBlockLocator, {
    Self {
        accepting_chain_block: RpcHash::from_str(&item.accepting_chain_block)?,
        transaction_ids: item.transaction_ids.iter().map(|x| RpcHash::from_str(x)).collect::<Result<Vec<_>, _>>()?,
    }
});

try_from!(item: &protowire::RpcTransactionInclusionIndicesLocator, kaspa_rpc_core::RpcTransactionInclusionIndicesLocator, {
    Self {
        block_hash: RpcHash::from_str(&item.block_hash)?,
        indices_within_block: item.indices_within_block.to_owned(),
    }
});

try_from!(item: &protowire::RpcTransactionAcceptingDaaScoreLocator, kaspa_rpc_core::RpcTransactionAcceptingDaaScoreLocator, {
    Self {
        accepting_daa_score: item.accepting_daa_score,
        transaction_ids: item.transaction_ids.iter().map(|x| RpcHash::from_str(x)).collect::<Result<Vec<_>, _>>()?,
    }
});

try_from!(item: &protowire::RpcTransactionLocator, kaspa_rpc_core::RpcTransactionLocator, {
    match item.transaction_locator {
        Some(protowire::rpc_transaction_locator::TransactionLocator::ByAcceptingBlock(ref transaction_locator)) => {
            Ok(kaspa_rpc_core::RpcTransactionLocator::ByAcceptingBlock(transaction_locator.try_into()?))
        },
        Some(protowire::rpc_transaction_locator::TransactionLocator::ByInclusionIndices(ref transaction_locator)) => {
            Ok(kaspa_rpc_core::RpcTransactionLocator::ByInclusionIndices(transaction_locator.try_into()?))
        },
        Some(protowire::rpc_transaction_locator::TransactionLocator::ByAcceptingDaaScore(ref transaction_locator)) => {
            Ok(kaspa_rpc_core::RpcTransactionLocator::ByAcceptingDaaScore(transaction_locator.try_into()?))
        },
        None => Err(RpcError::MissingRpcFieldError("RpcTransactionLocator".to_string(), "TransactionLocator".to_string())),
    }?
});
