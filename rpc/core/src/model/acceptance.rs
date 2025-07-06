use crate::{RpcHeader, RpcTransaction};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use workflow_serializer::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcAcceptanceData {
    pub accepting_chain_block_header: RpcHeader,
    pub mergeset_block_acceptance_data: Vec<RpcMergesetBlockAcceptanceData>,
}

impl RpcAcceptanceData {
    pub fn new(
        accepting_chain_block_header: RpcHeader,
        mergeset_block_acceptance_data: Vec<RpcMergesetBlockAcceptanceData>,
    ) -> Self {
        Self {
            accepting_chain_block_header,
            mergeset_block_acceptance_data,
        }
    }
}

impl Serializer for RpcAcceptanceData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u16, &1, writer)?;
        store!(RpcHeader, &self.accepting_chain_block_header, writer)?;
        store!(Vec<RpcMergesetBlockAcceptanceData>, &self.mergeset_block_acceptance_data, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcAcceptanceData {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u16, reader)?;
        let accepting_chain_block_header = load!(RpcHeader, reader)?;
        let mergeset_block_acceptance_data = load!(Vec<RpcMergesetBlockAcceptanceData>, reader)?;

        Ok(Self { accepting_chain_block_header, mergeset_block_acceptance_data })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcMergesetBlockAcceptanceData {
    pub merged_header: RpcHeader,
    pub accepted_transactions: Vec<RpcTransaction>,
}

impl RpcMergesetBlockAcceptanceData {
    pub fn new(
        merged_header: RpcHeader,
        accepted_transactions: Vec<RpcTransaction>,
    ) -> Self {
        Self {
            merged_header,
            accepted_transactions,
        }
    }
}

impl Serializer for RpcMergesetBlockAcceptanceData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u16, &1, writer)?;
        store!(RpcHeader, &self.merged_header, writer)?;
        store!(Vec<RpcTransaction>, &self.accepted_transactions, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcMergesetBlockAcceptanceData {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u16, reader)?;
        let merged_header = load!(RpcHeader, reader)?;
        let accepted_transactions = load!(Vec<RpcTransaction>, reader)?;

        Ok(Self { merged_header, accepted_transactions })
    }
}