use kaspa_consensus_client::{Header, Transaction};
use kaspa_consensus_core::acceptance_data::{AcceptanceData, MergesetBlockAcceptanceData};
use kaspa_hashes::Hash;
use serde::{Deserialize, Serialize};
use workflow_serializer::prelude::*;
use crate::prelude::{RpcBlockHeader, RpcTransaction};

use super::{RpcBlockHeaderVerbosity, RpcTransactionVerbosity};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcAcceptanceData {
    pub accepting_blue_score: Option<u64>,
    pub mergeset_block_acceptance_data: Vec<RpcMergesetBlockAcceptanceData>,
}

impl RpcAcceptanceData {
    pub fn new(
        accepting_blue_score: Option<u64>,
        mergeset_block_acceptance_data: Vec<RpcMergesetBlockAcceptanceData>,
    ) -> Self {
        Self {
            accepting_blue_score,
            mergeset_block_acceptance_data,
        }
    }
}

impl Serialize for RpcAcceptanceData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer);
        store!(Option<u64>, &self.accepting_blue_score, writer);
        store!(Option<RpcBlockHeader>, &self.accepting_chain_block_header, writer);
        store!(Vec<RpcMergesetBlockAcceptanceData>, &self.mergeset_block_acceptance_data, writer);

        Ok(())
    }
}

impl Deserialize for RpcAcceptanceData {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader);
        let accepting_blue_score = load!(Option<u64>, reader)?;
        let accepting_chain_block_header = load!(Option<RpcBlockHeader>, reader)?;
        let mergeset_block_acceptance_data = load!(Vec<RpcMergesetBlockAcceptanceData>, reader)?;

        Ok(Self {
            accepting_blue_score,
            mergeset_block_acceptance_data,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcMergesetBlockAcceptanceData {
    pub hash: Option<String>,
    pub header: Option<RpcBlockHeader>,
    pub accepted_transactions: Vec<RpcTransaction>,
}

impl RpcMergesetBlockAcceptanceData {
    #[inline(always)]
    pub fn new(
        hash: Option<String>,
        header: Option<RpcBlockHeader>,
        accepted_transactions: Vec<RpcTransaction>,
    ) -> Self {
        Self {
            hash,
            header,
            accepted_transactions,
        }
    }
}

impl Serialize for RpcMergesetBlockAcceptanceData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store(u8, &1, writer);

        store!(Option<String>, &self.hash, writer);
        store!(Option<RpcBlockHeader>, &self.header, writer);
        store!(Vec<RpcTransaction>, &self.accepted_transactions, writer);

        Ok(())
    }
}

impl Deserialize for RpcMergesetBlockAcceptanceData {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader);

        let hash = load!(Option<String>, reader)?;
        let header = load!(Option<RpcBlockHeader>, reader)?;
        let accepted_transactions = load!(Vec<RpcTransaction>, reader)?;

        Ok(Self {
            hash,
            header,
            accepted_transactions,
        })
    }
}

pub struct RpcAcceptanceDataVerbosity {
    pub include_accepting_blue_score: bool,
    pub accepting_chain_block_header_verbosity: Option<RpcBlockHeaderVerbosity>,
    pub mergeset_block_acceptance_verbosity: Option<RpcMergesetBlockAcceptanceVerbosity>,
}

impl RpcAcceptanceDataVerbosity {
    pub fn new(
        include_accepting_blue_score: bool,
        accepting_chain_block_header_verbosity: Option<RpcBlockHeaderVerbosity>,
        mergeset_block_acceptance_verbosity: Option<RpcMergesetBlockAcceptanceVerbosity>,
    ) -> Self {
        Self {
            include_accepting_blue_score,
            accepting_chain_block_header_verbosity,
            mergeset_block_acceptance_verbosity,
        }
    }
}

impl Serialize for RpcAcceptanceDataVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer);
        store!(bool, &self.include_accepting_blue_score, writer);
        serialize!(Option<RpcBlockHeaderVerbosity>, &self.accepting_chain_block_header_verbosity, writer);
        serialize!(Option<RpcMergesetBlockAcceptanceVerbosity>, &self.mergeset_block_acceptance_verbosity, writer);

        Ok(())
    }
}

impl Deserialize for RpcAcceptanceDataVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader);
        let include_accepting_blue_score = load!(bool, reader)?;
        let accepting_chain_block_header_verbosity = deserialize!(Option<RpcBlockHeaderVerbosity>, reader)?;
        let mergeset_block_acceptance_verbosity = deserialize!(Option<RpcMergesetBlockAcceptanceVerbosity>, reader)?;

        Ok(Self {
            include_accepting_blue_score,
            accepting_chain_block_header_verbosity,
            mergeset_block_acceptance_verbosity,
        })
    }
}

pub struct RpcMergesetBlockAcceptanceVerbosity {
    pub include_hash: bool,
    pub header_verbosity: Option<RpcBlockHeaderVerbosity>,
    pub accepted_transactions_verbosity: Option<RpcTransactionVerbosity>,
}

impl RpcMergesetBlockAcceptanceVerbosity {
    pub fn new(
        include_hash: bool,
        header_verbosity: Option<RpcBlockHeaderVerbosity>,
        accepted_transactions_verbosity: Option<RpcTransactionVerbosity>,
    ) -> Self {
        Self {
            include_hash,
            header_verbosity,
            accepted_transactions_verbosity,
        }
    }
}

impl Serialize for RpcMergesetBlockAcceptanceVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer);
        store!(bool, &self.include_hash, writer);
        serialize!(Option<RpcBlockHeaderVerbosity>, &self.header_verbosity, writer);
        serialize!(Option<RpcTransactionVerbosity>, &self.accepted_transactions_verbosity, writer);

        Ok(())
    }
}

impl Deserialize for RpcMergesetBlockAcceptanceVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader);
        let include_hash = load!(bool, reader)?;
        let header_verbosity = deserialize!(Option<RpcBlockHeaderVerbosity>, reader)?;
        let accepted_transactions_verbosity = deserialize!(Option<RpcTransactionVerbosity>, reader)?;

        Ok(Self {
            include_hash,
            header_verbosity,
            accepted_transactions_verbosity,
        })
    }
}
