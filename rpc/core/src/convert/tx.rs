//! Conversion of Transaction related types

use crate::{RpcError, RpcResult, RpcTransaction, RpcTransactionAcceptanceLocator, RpcTransactionInput, RpcTransactionLocator, RpcTransactionLocatorByAcceptance, RpcTransactionOutput};
use kaspa_consensus_core::tx::{Transaction, TransactionAcceptanceLocator, TransactionInclusionLocator, TransactionInput, TransactionLocator, TransactionOutput};

// ----------------------------------------------------------------------------
// consensus_core to rpc_core
// ----------------------------------------------------------------------------

impl From<&Transaction> for RpcTransaction {
    fn from(item: &Transaction) -> Self {
        Self {
            version: item.version,
            inputs: item.inputs.iter().map(RpcTransactionInput::from).collect(),
            outputs: item.outputs.iter().map(RpcTransactionOutput::from).collect(),
            lock_time: item.lock_time,
            subnetwork_id: item.subnetwork_id.clone(),
            gas: item.gas,
            payload: item.payload.clone(),
            mass: item.mass(),
            // TODO: Implement a populating process inspired from kaspad\app\rpc\rpccontext\verbosedata.go
            verbose_data: None,
        }
    }
}

impl From<&TransactionOutput> for RpcTransactionOutput {
    fn from(item: &TransactionOutput) -> Self {
        Self {
            value: item.value,
            script_public_key: item.script_public_key.clone(),
            // TODO: Implement a populating process inspired from kaspad\app\rpc\rpccontext\verbosedata.go
            verbose_data: None,
        }
    }
}

impl From<&TransactionInput> for RpcTransactionInput {
    fn from(item: &TransactionInput) -> Self {
        Self {
            previous_outpoint: item.previous_outpoint.into(),
            signature_script: item.signature_script.clone(),
            sequence: item.sequence,
            sig_op_count: item.sig_op_count,
            // TODO: Implement a populating process inspired from kaspad\app\rpc\rpccontext\verbosedata.go
            verbose_data: None,
        }
    }
}

impl From<TransactionLocator> for RpcTransactionLocator {
    #[inline(always)]
    fn from(item: TransactionLocator) -> Self {
        match item {
            TransactionLocator::ByAcceptance(transaction_acceptance_locator) => RpcTransactionLocator::ByAcceptance(transaction_acceptance_locator.into()),
            TransactionLocator::ByInclusion(transaction_inclusion_locator) => RpcTransactionLocator::ByInclusion(transaction_inclusion_locator.into()),
        }
    }
}

impl From<&TransactionAcceptanceLocator> for RpcTransactionAcceptanceLocator {
    #[inline(always)]
    fn from(item: TransactionAcceptanceLocator) -> Self {
        Self {
            chain_block: item.chain_block,
            transaction_ids: item.transaction_ids,
        }
    }
}

impl From<&TransactionInclusionLocator> for RpcTransactionInclusionLocator {
    #[inline(always)]
    fn from(item: TransactionInclusionLocator) -> Self {
        Self {
            block_hash: item.block_hash,
            indices_within_block: item.indices_within_block,
        }
    }
}

// ----------------------------------------------------------------------------
// rpc_core to consensus_core
// ----------------------------------------------------------------------------

impl TryFrom<RpcTransaction> for Transaction {
    type Error = RpcError;
    fn try_from(item: RpcTransaction) -> RpcResult<Self> {
        let transaction = Transaction::new(
            item.version,
            item.inputs
                .into_iter()
                .map(kaspa_consensus_core::tx::TransactionInput::try_from)
                .collect::<RpcResult<Vec<kaspa_consensus_core::tx::TransactionInput>>>()?,
            item.outputs
                .into_iter()
                .map(kaspa_consensus_core::tx::TransactionOutput::try_from)
                .collect::<RpcResult<Vec<kaspa_consensus_core::tx::TransactionOutput>>>()?,
            item.lock_time,
            item.subnetwork_id.clone(),
            item.gas,
            item.payload.clone(),
        );
        transaction.set_mass(item.mass);
        Ok(transaction)
    }
}

impl TryFrom<RpcTransactionOutput> for TransactionOutput {
    type Error = RpcError;
    fn try_from(item: RpcTransactionOutput) -> RpcResult<Self> {
        Ok(Self::new(item.value, item.script_public_key))
    }
}

impl TryFrom<RpcTransactionInput> for TransactionInput {
    type Error = RpcError;
    fn try_from(item: RpcTransactionInput) -> RpcResult<Self> {
        Ok(Self::new(item.previous_outpoint.into(), item.signature_script, item.sequence, item.sig_op_count))
    }
}

impl TryFrom<RpcTransactionLocator> for TransactionLocator {
    type Error = RpcError;
    fn try_from(item: RpcTransactionLocator) -> RpcResult<Self> {
        match item {
            RpcTransactionLocator::ByAcceptance(transaction_acceptance_locator) => Ok(TransactionLocator::ByAcceptance(transaction_acceptance_locator.into())),
            RpcTransactionLocator::ByInclusion(transaction_inclusion_locator) => Ok(TransactionLocator::ByInclusion(transaction_inclusion_locator.into())),
        }
    }
}

impl TryFrom<RpcTransactionAcceptanceLocator> for TransactionAcceptanceLocator {
    type Error = RpcError;
    fn try_from(item: RpcTransactionAcceptanceLocator) -> RpcResult<Self> {
        Ok(Self {
            chain_hash: item.chain_block,
            transaction_ids: item.accepted_transaction_ids,
        })
    }
}

impl TryFrom<RpcTransactionInclusionLocator> for TransactionInclusionLocator {
    type Error = RpcError;
    fn try_from(item: RpcTransactionInclusionLocator) -> RpcResult<Self> {
        Ok(Self {
            block_hash: item.block_hash,
            indices_within_block: item.indices_within_block,
        })
    }
}