//! Conversion of Transaction related types

use crate::{
    RpcError, RpcResult, RpcTransaction, RpcTransactionAcceptanceLocator, RpcTransactionInclusionLocator, RpcTransactionInput,
    RpcTransactionLocator, RpcTransactionOutput,
};
use kaspa_consensus_core::tx::{
    Transaction, TransactionAcceptanceLocator, TransactionInclusionLocator, TransactionInput, TransactionLocator, TransactionOutput,
};

// ----------------------------------------------------------------------------
// consensus_core to rpc_core
// ----------------------------------------------------------------------------

impl From<&Transaction> for RpcTransaction {
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn from(item: &TransactionInput) -> Self {
        Self {
            previous_outpoint: Some(item.previous_outpoint.into()),
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
            TransactionLocator::ByAcceptance(transaction_acceptance_locator) => {
                RpcTransactionLocator::ByAcceptance(RpcTransactionAcceptanceLocator::from(transaction_acceptance_locator))
            }
            TransactionLocator::ByInclusion(transaction_inclusion_locator) => {
                RpcTransactionLocator::ByInclusion(RpcTransactionInclusionLocator::from(transaction_inclusion_locator))
            }
        }
    }
}

impl From<TransactionAcceptanceLocator> for RpcTransactionAcceptanceLocator {
    #[inline(always)]
    fn from(item: TransactionAcceptanceLocator) -> Self {
        Self {
            accepting_chain_block: item.accepting_chain_block,
            transaction_ids: item.transaction_ids.map_or(vec![], |transaction_ids| transaction_ids),
        }
    }
}

impl From<TransactionInclusionLocator> for RpcTransactionInclusionLocator {
    #[inline(always)]
    fn from(item: TransactionInclusionLocator) -> Self {
        Self {
            block_hash: item.block_hash,
            indices_within_block: item.indices_within_block.map_or(vec![], |indices_within_block| indices_within_block),
        }
    }
}

// ----------------------------------------------------------------------------
// rpc_core to consensus_core
// ----------------------------------------------------------------------------

impl TryFrom<RpcTransaction> for Transaction {
    type Error = RpcError;
    #[inline(always)]
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
    #[inline(always)]
    fn try_from(item: RpcTransactionOutput) -> RpcResult<Self> {
        Ok(Self::new(item.value, item.script_public_key))
    }
}

impl TryFrom<RpcTransactionInput> for TransactionInput {
    type Error = RpcError;
    #[inline(always)]
    fn try_from(item: RpcTransactionInput) -> RpcResult<Self> {
        Ok(Self::new(
            item.previous_outpoint.ok_or(RpcError::General("rpc struct missing previous outpoint".to_owned()))?.into(),
            item.signature_script,
            item.sequence,
            item.sig_op_count,
        ))
    }
}

impl TryFrom<RpcTransactionLocator> for TransactionLocator {
    type Error = RpcError;
    #[inline(always)]
    fn try_from(item: RpcTransactionLocator) -> RpcResult<Self> {
        match item {
            RpcTransactionLocator::ByAcceptance(transaction_acceptance_locator) => {
                Ok(TransactionLocator::ByAcceptance(transaction_acceptance_locator.try_into()?))
            }
            RpcTransactionLocator::ByInclusion(transaction_inclusion_locator) => {
                Ok(TransactionLocator::ByInclusion(transaction_inclusion_locator.try_into()?))
            }
        }
    }
}

impl TryFrom<RpcTransactionAcceptanceLocator> for TransactionAcceptanceLocator {
    type Error = RpcError;
    #[inline(always)]
    fn try_from(item: RpcTransactionAcceptanceLocator) -> RpcResult<Self> {
        Ok(Self {
            accepting_chain_block: item.accepting_chain_block,
            transaction_ids: (!item.transaction_ids.is_empty()).then(|| item.transaction_ids),
        })
    }
}

impl TryFrom<RpcTransactionInclusionLocator> for TransactionInclusionLocator {
    type Error = RpcError;
    #[inline(always)]
    fn try_from(item: RpcTransactionInclusionLocator) -> RpcResult<Self> {
        Ok(Self {
            block_hash: item.block_hash,
            indices_within_block: (!item.indices_within_block.is_empty()).then(|| item.indices_within_block),
        })
    }
}
