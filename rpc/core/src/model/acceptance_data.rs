use serde::{Deserialize, Serialize};
use workflow_serializer::prelude::*;

use super::{RpcHash, RpcHeader, RpcHeaderVerbosity, RpcTransaction, RpcTransactionVerbosity};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcAcceptanceData {
    pub accepting_blue_score: Option<u64>,
    pub mergeset_block_acceptance_data: Vec<RpcMergesetBlockAcceptanceData>,
}

impl RpcAcceptanceData {
    pub fn new(accepting_blue_score: Option<u64>, mergeset_block_acceptance_data: Vec<RpcMergesetBlockAcceptanceData>) -> Self {
        Self { accepting_blue_score, mergeset_block_acceptance_data }
    }
}

impl Serializer for RpcAcceptanceData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(Option<u64>, &self.accepting_blue_score, writer)?;
        serialize!(Vec<RpcMergesetBlockAcceptanceData>, &self.mergeset_block_acceptance_data, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcAcceptanceData {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader);
        let accepting_blue_score = load!(Option<u64>, reader)?;
        let mergeset_block_acceptance_data = deserialize!(Vec<RpcMergesetBlockAcceptanceData>, reader)?;

        Ok(Self { accepting_blue_score, mergeset_block_acceptance_data })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcMergesetBlockAcceptanceData {
    pub hash: Option<RpcHash>,
    pub header: Option<RpcHeader>,
    pub accepted_transactions: Vec<RpcTransaction>,
}

impl RpcMergesetBlockAcceptanceData {
    #[inline(always)]
    pub fn new(hash: Option<RpcHash>, header: Option<RpcHeader>, accepted_transactions: Vec<RpcTransaction>) -> Self {
        Self { hash, header, accepted_transactions }
    }
}

impl Serializer for RpcMergesetBlockAcceptanceData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;

        store!(Option<RpcHash>, &self.hash, writer)?;
        store!(Option<RpcHeader>, &self.header, writer)?;
        serialize!(Vec<RpcTransaction>, &self.accepted_transactions, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcMergesetBlockAcceptanceData {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader);

        let hash = load!(Option<RpcHash>, reader)?;
        let header = load!(Option<RpcHeader>, reader)?;
        let accepted_transactions = deserialize!(Vec<RpcTransaction>, reader)?;

        Ok(Self { hash, header, accepted_transactions })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcAcceptanceDataVerbosity {
    pub include_accepting_blue_score: Option<bool>,
    pub mergeset_block_acceptance_data_verbosity: Option<RpcMergesetBlockAcceptanceDataVerbosity>,
}

impl RpcAcceptanceDataVerbosity {
    pub fn new(
        include_accepting_blue_score: Option<bool>,
        mergeset_block_acceptance_data_verbosity: Option<RpcMergesetBlockAcceptanceDataVerbosity>,
    ) -> Self {
        Self { include_accepting_blue_score, mergeset_block_acceptance_data_verbosity }
    }
}

impl Serializer for RpcAcceptanceDataVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(Option<bool>, &self.include_accepting_blue_score, writer)?;
        serialize!(Option<RpcMergesetBlockAcceptanceDataVerbosity>, &self.mergeset_block_acceptance_data_verbosity, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcAcceptanceDataVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader);
        let include_accepting_blue_score = load!(Option<bool>, reader)?;
        let mergeset_block_acceptance_data_verbosity = deserialize!(Option<RpcMergesetBlockAcceptanceDataVerbosity>, reader)?;

        Ok(Self { include_accepting_blue_score, mergeset_block_acceptance_data_verbosity })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcMergesetBlockAcceptanceDataVerbosity {
    pub include_hash: Option<bool>,
    pub header_verbosity: Option<RpcHeaderVerbosity>,
    pub accepted_transactions_verbosity: Option<RpcTransactionVerbosity>,
}

impl RpcMergesetBlockAcceptanceDataVerbosity {
    pub fn new(
        include_hash: Option<bool>,
        header_verbosity: Option<RpcHeaderVerbosity>,
        accepted_transactions_verbosity: Option<RpcTransactionVerbosity>,
    ) -> Self {
        Self { include_hash, header_verbosity, accepted_transactions_verbosity }
    }
}

impl Serializer for RpcMergesetBlockAcceptanceDataVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(Option<bool>, &self.include_hash, writer)?;
        serialize!(Option<RpcHeaderVerbosity>, &self.header_verbosity, writer)?;
        serialize!(Option<RpcTransactionVerbosity>, &self.accepted_transactions_verbosity, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcMergesetBlockAcceptanceDataVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let include_hash = load!(Option<bool>, reader)?;
        let header_verbosity = deserialize!(Option<RpcHeaderVerbosity>, reader)?;
        let accepted_transactions_verbosity = deserialize!(Option<RpcTransactionVerbosity>, reader)?;

        Ok(Self { include_hash, header_verbosity, accepted_transactions_verbosity })
    }
}
