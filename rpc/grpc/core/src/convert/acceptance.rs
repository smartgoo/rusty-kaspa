use crate::protowire;
use crate::{from, try_from};
use kaspa_rpc_core::{RpcError, RpcMergesetBlockAcceptanceData};

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

from!(item: &kaspa_rpc_core::RpcAcceptanceData, protowire::RpcAcceptanceData, {
    Self {
        accepting_chain_block_header: Some(protowire::RpcBlockHeader::from(&item.accepting_chain_block_header)),
        mergeset_block_acceptance_data: item
            .mergeset_block_acceptance_data
            .iter()
            .map(protowire::RpcMergesetBlockAcceptanceData::from)
            .collect(),
    }
});

from!(item: &kaspa_rpc_core::RpcMergesetBlockAcceptanceData, protowire::RpcMergesetBlockAcceptanceData, {
    Self {
        merged_header: Some(protowire::RpcBlockHeader::from(&item.merged_header)),
        accepted_transactions: item.accepted_transactions.iter().map(protowire::RpcTransaction::from).collect(),
    }
});

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

try_from!(item: &protowire::RpcAcceptanceData, kaspa_rpc_core::RpcAcceptanceData, {
    Self {
        accepting_chain_block_header: item
            .accepting_chain_block_header
            .as_ref()
            .map(kaspa_rpc_core::RpcHeader::try_from)
            .transpose()?
            .unwrap(),
        mergeset_block_acceptance_data: item
            .mergeset_block_acceptance_data
            .iter()
            .map(RpcMergesetBlockAcceptanceData::try_from)
            .collect::<Result<_, _>>()?,
    }
});

try_from!(item: &protowire::RpcMergesetBlockAcceptanceData, kaspa_rpc_core::RpcMergesetBlockAcceptanceData, {
    Self {
        merged_header: item.merged_header.as_ref().map(kaspa_rpc_core::RpcHeader::try_from).transpose()?.unwrap(),
        accepted_transactions: item.accepted_transactions.iter().map(kaspa_rpc_core::RpcTransaction::try_from).collect::<Result<_, _>>()?,
    }
});
