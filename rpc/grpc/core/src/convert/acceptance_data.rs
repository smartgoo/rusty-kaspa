use crate::protowire::{self, RpcBlockHeaderVerbosity, RpcTransactionVerbosity};
use crate::{from, try_from};
use kaspa_rpc_core::{RpcError, RpcMergesetBlockAcceptanceData};
use std::str::FromStr;

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

from!(item: &kaspa_rpc_core::RpcAcceptanceData,  protowire::RpcAcceptanceData, {
    Self {
        accepting_blue_score: item.accepting_blue_score,
        mergeset_block_acceptance_data: item
            .mergeset_block_acceptance_data
            .iter()
            .map(|x| protowire::RpcMergesetBlockAcceptanceData::from(x))
            .collect(),
    }
});

from!(item: &kaspa_rpc_core::RpcAcceptanceDataVerbosity, protowire::RpcAcceptanceDataVerbosity, {
    Self {
        include_accepting_blue_score: item.include_accepting_blue_score,
        mergeset_block_acceptance_data_verbosity: item.mergeset_block_acceptance_data_verbosity.as_ref().map(|x| protowire::RpcMergesetBlockAcceptanceDataVerbosity::from(x)),
    }
});

from!(item: &kaspa_rpc_core::RpcMergesetBlockAcceptanceData, protowire::RpcMergesetBlockAcceptanceData, {
    Self {
        hash: item.hash.as_ref().map(|x| x.to_string()),
        header: item.header.as_ref().map(|x| protowire::RpcBlockHeader::from(x)),
        accepted_transactions: item.accepted_transactions.iter().map(|x| protowire::RpcTransaction::from(x)).collect(),
    }
});

from!(item: &kaspa_rpc_core::RpcMergesetBlockAcceptanceDataVerbosity, protowire::RpcMergesetBlockAcceptanceDataVerbosity, {
    Self {
        include_hash: item.include_hash,
        header_verbosity: item.header_verbosity.as_ref().map(|v| RpcBlockHeaderVerbosity::from(v)),
        accepted_transactions_verbosity: item.accepted_transactions_verbosity.as_ref().map(|v| RpcTransactionVerbosity::from(v)),
    }
});

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

try_from!(item: &protowire::RpcAcceptanceData, kaspa_rpc_core::RpcAcceptanceData, {
    Self {
        accepting_blue_score: item.accepting_blue_score,
        mergeset_block_acceptance_data: item
        .mergeset_block_acceptance_data
        .iter()
        .map(|v| RpcMergesetBlockAcceptanceData::try_from(v))
        .collect::<Result<_, _>>()?,
    }
});

try_from!(item: &protowire::RpcAcceptanceDataVerbosity, kaspa_rpc_core::RpcAcceptanceDataVerbosity, {
    Self {
        include_accepting_blue_score: item.include_accepting_blue_score,
        mergeset_block_acceptance_data_verbosity: item.mergeset_block_acceptance_data_verbosity.as_ref().map(|v| kaspa_rpc_core::RpcMergesetBlockAcceptanceDataVerbosity::try_from(v)).transpose()?,
    }
});

try_from!(item: &protowire::RpcMergesetBlockAcceptanceData, kaspa_rpc_core::RpcMergesetBlockAcceptanceData, {
    Self {
        hash: item.hash.as_ref().map(|x| kaspa_rpc_core::RpcHash::from_str(x)).transpose()?,
        header: item.header.as_ref().map(|x| kaspa_rpc_core::RpcHeader::try_from(x)).transpose()?,
        accepted_transactions: item.accepted_transactions.iter().map(|x| kaspa_rpc_core::RpcTransaction::try_from(x)).collect::<Result<_, _>>()?,
    }
});

try_from!(item: &protowire::RpcMergesetBlockAcceptanceDataVerbosity, kaspa_rpc_core::RpcMergesetBlockAcceptanceDataVerbosity, {
    Self {
        include_hash: item.include_hash,
        header_verbosity: item.header_verbosity.as_ref().map(|x| kaspa_rpc_core::RpcHeaderVerbosity::try_from(x)).transpose()?,
        accepted_transactions_verbosity: item.accepted_transactions_verbosity.as_ref().map(|x| kaspa_rpc_core::RpcTransactionVerbosity::try_from(x)).transpose()?,
    }
});
