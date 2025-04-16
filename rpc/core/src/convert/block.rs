//! Conversion of Block related types

use std::sync::Arc;

use crate::{RpcBlock, RpcError, RpcHeader, RpcRawBlock, RpcRawHeader, RpcResult, RpcTransaction};
use kaspa_consensus_core::block::{Block, MutableBlock};

// ----------------------------------------------------------------------------
// consensus_core to rpc_core
// ----------------------------------------------------------------------------

impl From<&Block> for RpcBlock {
    fn from(item: &Block) -> Self {
        Self {
            header: Some(item.header.as_ref().into()),
            transactions: item.transactions.iter().map(RpcTransaction::from).collect(),
            // TODO: Implement a populating process inspired from kaspad\app\rpc\rpccontext\verbosedata.go
            verbose_data: None,
        }
    }
}

impl From<&Block> for RpcRawBlock {
    fn from(item: &Block) -> Self {
        Self {
            header: RpcRawHeader::from(item.header.as_ref()),
            transactions: item.transactions.iter().map(RpcTransaction::from).collect(),
        }
    }
}

impl From<&MutableBlock> for RpcBlock {
    fn from(item: &MutableBlock) -> Self {
        Self {
            header: Some(RpcHeader::from(item.header.as_ref())),
            transactions: item.transactions.iter().map(RpcTransaction::from).collect(),
            verbose_data: None,
        }
    }
}

impl From<&MutableBlock> for RpcRawBlock {
    fn from(item: &MutableBlock) -> Self {
        Self { header: RpcRawHeader::from(&item.header), transactions: item.transactions.iter().map(RpcTransaction::from).collect() }
    }
}

impl From<MutableBlock> for RpcRawBlock {
    fn from(item: MutableBlock) -> Self {
        Self { header: RpcRawHeader::from(item.header), transactions: item.transactions.iter().map(RpcTransaction::from).collect() }
    }
}

// ----------------------------------------------------------------------------
// rpc_core to consensus_core
// ----------------------------------------------------------------------------

/*
impl TryFrom<RpcHeader> for kaspa_consensus_core::header::Header {
    type Error = RpcError;
    fn try_from(item: RpcHeader) -> RpcResult<Self> {
        Ok(Self {
            hash: item.hash.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "hash".to_string()))?,
            version: item.version.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "version".to_string()))?,
            parents_by_level: item.parents_by_level,
            hash_merkle_root: item.hash_merkle_root.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "hash_merkle_root".to_string()))?,
            accepted_id_merkle_root: item.accepted_id_merkle_root.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "accepted_id_merkle_root".to_string()))?,
            utxo_commitment: item.utxo_commitment.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "utxo_commitment".to_string()))?,
            timestamp: item.timestamp.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "timestamp".to_string()))?,
            bits: item.bits.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "bits".to_string()))?,
            nonce: item.nonce.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "nonce".to_string()))?,
            daa_score: item.daa_score.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "daa_score".to_string()))?,
            blue_work: item.blue_work.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "blue_work".to_string()))?,
            blue_score: item.blue_score.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "blue_score".to_string()))?,
            pruning_point: item.pruning_point.ok_or(RpcError::MissingRpcFieldError("RpcHeader".to_string(), "pruning_point".to_string()))?,
        })
    }
}
*/

impl TryFrom<RpcBlock> for Block {
    type Error = RpcError;
    fn try_from(item: RpcBlock) -> RpcResult<Self> {
        Ok(Self {
            header: Arc::new(
                (item.header.ok_or(RpcError::MissingRpcFieldError("RpcBlock".to_string(), "header".to_string()))?).try_into()?,
            ),
            transactions: Arc::new(
                item.transactions
                    .into_iter()
                    .map(kaspa_consensus_core::tx::Transaction::try_from)
                    .collect::<RpcResult<Vec<kaspa_consensus_core::tx::Transaction>>>()?,
            ),
        })
    }
}

impl TryFrom<RpcRawBlock> for Block {
    type Error = RpcError;
    fn try_from(item: RpcRawBlock) -> RpcResult<Self> {
        Ok(Self {
            header: Arc::new(kaspa_consensus_core::header::Header::from(item.header)),
            transactions: Arc::new(
                item.transactions
                    .into_iter()
                    .map(kaspa_consensus_core::tx::Transaction::try_from)
                    .collect::<RpcResult<Vec<kaspa_consensus_core::tx::Transaction>>>()?,
            ),
        })
    }
}
