use borsh::{BorshDeserialize, BorshSerialize};
use kaspa_addresses::Address;
use kaspa_consensus_core::tx::{
    ScriptPublicKey, ScriptVec, TransactionId, TransactionIndexType, TransactionInput, TransactionOutpoint, TransactionOutput,
    UtxoEntry,
};
use kaspa_utils::{hex::ToHex, serde_bytes_fixed_ref};
use serde::{Deserialize, Serialize};
use workflow_serializer::prelude::*;

use crate::prelude::{RpcHash, RpcScriptClass, RpcSubnetworkId};

/// Represents the ID of a Kaspa transaction
pub type RpcTransactionId = TransactionId;

pub type RpcScriptVec = ScriptVec;
pub type RpcScriptPublicKey = ScriptPublicKey;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcUtxoEntry {
    pub amount: u64,
    pub script_public_key: ScriptPublicKey,
    pub block_daa_score: u64,
    pub is_coinbase: bool,
}

impl RpcUtxoEntry {
    pub fn new(amount: u64, script_public_key: ScriptPublicKey, block_daa_score: u64, is_coinbase: bool) -> Self {
        Self { amount, script_public_key, block_daa_score, is_coinbase }
    }
}

impl From<UtxoEntry> for RpcUtxoEntry {
    fn from(entry: UtxoEntry) -> Self {
        Self {
            amount: entry.amount,
            script_public_key: entry.script_public_key,
            block_daa_score: entry.block_daa_score,
            is_coinbase: entry.is_coinbase,
        }
    }
}

impl From<RpcUtxoEntry> for UtxoEntry {
    fn from(entry: RpcUtxoEntry) -> Self {
        Self {
            amount: entry.amount,
            script_public_key: entry.script_public_key,
            block_daa_score: entry.block_daa_score,
            is_coinbase: entry.is_coinbase,
        }
    }
}

impl Serializer for RpcUtxoEntry {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(u64, &self.amount, writer)?;
        store!(ScriptPublicKey, &self.script_public_key, writer)?;
        store!(u64, &self.block_daa_score, writer)?;
        store!(bool, &self.is_coinbase, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcUtxoEntry {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let amount = load!(u64, reader)?;
        let script_public_key = load!(ScriptPublicKey, reader)?;
        let block_daa_score = load!(u64, reader)?;
        let is_coinbase = load!(bool, reader)?;

        Ok(Self { amount, script_public_key, block_daa_score, is_coinbase })
    }
}

/// Represents a Kaspa transaction outpoint
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionOutpoint {
    #[serde(with = "serde_bytes_fixed_ref")]
    pub transaction_id: TransactionId,
    pub index: TransactionIndexType,
}

impl From<TransactionOutpoint> for RpcTransactionOutpoint {
    fn from(outpoint: TransactionOutpoint) -> Self {
        Self { transaction_id: outpoint.transaction_id, index: outpoint.index }
    }
}

impl From<RpcTransactionOutpoint> for TransactionOutpoint {
    fn from(outpoint: RpcTransactionOutpoint) -> Self {
        Self { transaction_id: outpoint.transaction_id, index: outpoint.index }
    }
}

impl From<kaspa_consensus_client::TransactionOutpoint> for RpcTransactionOutpoint {
    fn from(outpoint: kaspa_consensus_client::TransactionOutpoint) -> Self {
        TransactionOutpoint::from(outpoint).into()
    }
}

impl From<RpcTransactionOutpoint> for kaspa_consensus_client::TransactionOutpoint {
    fn from(outpoint: RpcTransactionOutpoint) -> Self {
        TransactionOutpoint::from(outpoint).into()
    }
}

impl Serializer for RpcTransactionOutpoint {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(TransactionId, &self.transaction_id, writer)?;
        store!(TransactionIndexType, &self.index, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionOutpoint {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let transaction_id = load!(TransactionId, reader)?;
        let index = load!(TransactionIndexType, reader)?;

        Ok(Self { transaction_id, index })
    }
}

/// Represents a Kaspa transaction input
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionInput {
    pub previous_outpoint: Option<RpcTransactionOutpoint>,
    #[serde(with = "hex::serde")]
    pub signature_script: Vec<u8>,
    pub sequence: u64,
    pub sig_op_count: u8,
    pub verbose_data: Option<RpcTransactionInputVerboseData>,
}

impl std::fmt::Debug for RpcTransactionInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpcTransactionInput")
            .field("previous_outpoint", &self.previous_outpoint)
            .field("signature_script", &self.signature_script.to_hex())
            .field("sequence", &self.sequence)
            .field("sig_op_count", &self.sig_op_count)
            .field("verbose_data", &self.verbose_data)
            .finish()
    }
}

impl From<TransactionInput> for RpcTransactionInput {
    fn from(input: TransactionInput) -> Self {
        Self {
            previous_outpoint: Some(input.previous_outpoint.into()),
            signature_script: input.signature_script,
            sequence: input.sequence,
            sig_op_count: input.sig_op_count,
            verbose_data: None,
        }
    }
}

impl RpcTransactionInput {
    pub fn from_transaction_inputs(other: Vec<TransactionInput>) -> Vec<Self> {
        other.into_iter().map(Self::from).collect()
    }
}

impl Serializer for RpcTransactionInput {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        serialize!(Option<RpcTransactionOutpoint>, &self.previous_outpoint, writer)?;
        store!(Vec<u8>, &self.signature_script, writer)?;
        store!(u64, &self.sequence, writer)?;
        store!(u8, &self.sig_op_count, writer)?;
        serialize!(Option<RpcTransactionInputVerboseData>, &self.verbose_data, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionInput {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let previous_outpoint = deserialize!(Option<RpcTransactionOutpoint>, reader)?;
        let signature_script = load!(Vec<u8>, reader)?;
        let sequence = load!(u64, reader)?;
        let sig_op_count = load!(u8, reader)?;
        let verbose_data = deserialize!(Option<RpcTransactionInputVerboseData>, reader)?;

        Ok(Self { previous_outpoint, signature_script, sequence, sig_op_count, verbose_data })
    }
}

/// Represent Kaspa transaction input verbose data
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionInputVerboseData {
    pub utxo_entry: Option<RpcUtxoEntry>,
}

impl Serializer for RpcTransactionInputVerboseData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        serialize!(Option<RpcUtxoEntry>, &self.utxo_entry, writer)?;
        Ok(())
    }
}

impl Deserializer for RpcTransactionInputVerboseData {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let utxo_entry = deserialize!(Option<RpcUtxoEntry>, reader)?;
        Ok(Self { utxo_entry })
    }
}

/// Represents a Kaspad transaction output
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionOutput {
    pub value: u64,
    pub script_public_key: RpcScriptPublicKey,
    pub verbose_data: Option<RpcTransactionOutputVerboseData>,
}

impl RpcTransactionOutput {
    pub fn from_transaction_outputs(other: Vec<TransactionOutput>) -> Vec<Self> {
        other.into_iter().map(Self::from).collect()
    }
}

impl From<TransactionOutput> for RpcTransactionOutput {
    fn from(output: TransactionOutput) -> Self {
        Self { value: output.value, script_public_key: output.script_public_key, verbose_data: None }
    }
}

impl Serializer for RpcTransactionOutput {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(u64, &self.value, writer)?;
        store!(RpcScriptPublicKey, &self.script_public_key, writer)?;
        serialize!(Option<RpcTransactionOutputVerboseData>, &self.verbose_data, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionOutput {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let value = load!(u64, reader)?;
        let script_public_key = load!(RpcScriptPublicKey, reader)?;
        let verbose_data = deserialize!(Option<RpcTransactionOutputVerboseData>, reader)?;

        Ok(Self { value, script_public_key, verbose_data })
    }
}

/// Represent Kaspa transaction output verbose data
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionOutputVerboseData {
    pub script_public_key_type: RpcScriptClass,
    pub script_public_key_address: Option<Address>,
}

impl Serializer for RpcTransactionOutputVerboseData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &2, writer)?;
        store!(RpcScriptClass, &self.script_public_key_type, writer)?;
        store!(Option<Address>, &self.script_public_key_address, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionOutputVerboseData {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let script_public_key_type = load!(RpcScriptClass, reader)?;
        let script_public_key_address = match _version {
            1 => Some(load!(Address, reader)?),
            2 => load!(Option<Address>, reader)?,
            _ => panic!("Invalid version for RpcTransactionOutputVerboseData"),
        };

        Ok(Self { script_public_key_type, script_public_key_address })
    }
}

/// Represents a Kaspa transaction
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransaction {
    pub version: u16,
    pub inputs: Vec<RpcTransactionInput>,
    pub outputs: Vec<RpcTransactionOutput>,
    pub lock_time: u64,
    pub subnetwork_id: RpcSubnetworkId,
    pub gas: u64,
    #[serde(with = "hex::serde")]
    pub payload: Vec<u8>,
    pub mass: u64,
    pub verbose_data: Option<RpcTransactionVerboseData>,
}

impl std::fmt::Debug for RpcTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpcTransaction")
            .field("version", &self.version)
            .field("lock_time", &self.lock_time)
            .field("subnetwork_id", &self.subnetwork_id)
            .field("gas", &self.gas)
            .field("payload", &self.payload.to_hex())
            .field("mass", &self.mass)
            .field("inputs", &self.inputs) // Inputs and outputs are placed purposely at the end for better debug visibility 
            .field("outputs", &self.outputs)
            .field("verbose_data", &self.verbose_data)
            .finish()
    }
}

impl Serializer for RpcTransaction {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u16, &1, writer)?;
        store!(u16, &self.version, writer)?;
        serialize!(Vec<RpcTransactionInput>, &self.inputs, writer)?;
        serialize!(Vec<RpcTransactionOutput>, &self.outputs, writer)?;
        store!(u64, &self.lock_time, writer)?;
        store!(RpcSubnetworkId, &self.subnetwork_id, writer)?;
        store!(u64, &self.gas, writer)?;
        store!(Vec<u8>, &self.payload, writer)?;
        store!(u64, &self.mass, writer)?;
        serialize!(Option<RpcTransactionVerboseData>, &self.verbose_data, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransaction {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _struct_version = load!(u16, reader)?;
        let version = load!(u16, reader)?;
        let inputs = deserialize!(Vec<RpcTransactionInput>, reader)?;
        let outputs = deserialize!(Vec<RpcTransactionOutput>, reader)?;
        let lock_time = load!(u64, reader)?;
        let subnetwork_id = load!(RpcSubnetworkId, reader)?;
        let gas = load!(u64, reader)?;
        let payload = load!(Vec<u8>, reader)?;
        let mass = load!(u64, reader)?;
        let verbose_data = deserialize!(Option<RpcTransactionVerboseData>, reader)?;

        Ok(Self { version, inputs, outputs, lock_time, subnetwork_id, gas, payload, mass, verbose_data })
    }
}

/// Represent Kaspa transaction verbose data
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionVerboseData {
    pub transaction_id: RpcTransactionId,
    pub hash: RpcHash,
    pub compute_mass: u64,
    pub block_hash: RpcHash,
    pub block_time: u64,
}

impl Serializer for RpcTransactionVerboseData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(RpcTransactionId, &self.transaction_id, writer)?;
        store!(RpcHash, &self.hash, writer)?;
        store!(u64, &self.compute_mass, writer)?;
        store!(RpcHash, &self.block_hash, writer)?;
        store!(u64, &self.block_time, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionVerboseData {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let transaction_id = load!(RpcTransactionId, reader)?;
        let hash = load!(RpcHash, reader)?;
        let compute_mass = load!(u64, reader)?;
        let block_hash = load!(RpcHash, reader)?;
        let block_time = load!(u64, reader)?;

        Ok(Self { transaction_id, hash, compute_mass, block_hash, block_time })
    }
}

/// Represents accepted transaction ids
#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcAcceptedTransactionIds {
    pub accepting_block_hash: RpcHash,
    pub accepted_transaction_ids: Vec<RpcTransactionId>,
}

/// Represents a transaction locator

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcTransactionLocator {
    ByAcceptingBlock(RpcTransactionAcceptingBlockLocator),
    ByAcceptingDaaScore(RpcTransactionAcceptingDaaScoreLocator),
    ByInclusionIndices(RpcTransactionInclusionIndicesLocator),
}

impl Serializer for RpcTransactionLocator {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(
            u8,
            &match self {
                RpcTransactionLocator::ByAcceptingBlock(_) => 1,
                RpcTransactionLocator::ByAcceptingDaaScore(_) => 2,
                RpcTransactionLocator::ByInclusionIndices(_) => 3,
            },
            writer
        )?;

        match self {
            RpcTransactionLocator::ByAcceptingBlock(ref locator) => {
                serialize!(RpcTransactionAcceptingBlockLocator, locator, writer)?;
            }
            RpcTransactionLocator::ByAcceptingDaaScore(ref locator) => {
                serialize!(RpcTransactionAcceptingDaaScoreLocator, locator, writer)?;
            }
            RpcTransactionLocator::ByInclusionIndices(ref locator) => {
                serialize!(RpcTransactionInclusionIndicesLocator, locator, writer)?;
            }
        }

        Ok(())
    }
}

impl Deserializer for RpcTransactionLocator {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let enum_type = load!(u8, reader)?;
        match enum_type {
            1 => {
                let locator = deserialize!(RpcTransactionAcceptingBlockLocator, reader)?;
                Ok(RpcTransactionLocator::ByAcceptingBlock(locator))
            }
            2 => {
                let locator = deserialize!(RpcTransactionAcceptingDaaScoreLocator, reader)?;
                Ok(RpcTransactionLocator::ByAcceptingDaaScore(locator))
            }
            3 => {
                let locator = deserialize!(RpcTransactionInclusionIndicesLocator, reader)?;
                Ok(RpcTransactionLocator::ByInclusionIndices(locator))
            }
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid enum_type for RpcTransactionLocator")),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionAcceptingBlockLocator {
    pub accepting_chain_block: RpcHash,
    pub transaction_ids: Vec<RpcTransactionId>,
}

impl RpcTransactionAcceptingBlockLocator {
    pub fn new(accepting_chain_block: RpcHash, transaction_ids: Vec<RpcTransactionId>) -> Self {
        Self { accepting_chain_block, transaction_ids }
    }
}

impl Serializer for RpcTransactionAcceptingBlockLocator {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(RpcHash, &self.accepting_chain_block, writer)?;
        store!(Vec<RpcTransactionId>, &self.transaction_ids, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionAcceptingBlockLocator {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let accepting_chain_block = load!(RpcHash, reader)?;
        let transaction_ids = load!(Vec<RpcTransactionId>, reader)?;

        Ok(Self { accepting_chain_block, transaction_ids })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionInclusionIndicesLocator {
    pub block_hash: RpcHash,
    pub indices_within_block: Vec<TransactionIndexType>,
}

impl RpcTransactionInclusionIndicesLocator {
    pub fn new(block_hash: RpcHash, indices_within_block: Vec<TransactionIndexType>) -> Self {
        Self { block_hash, indices_within_block }
    }
}

impl Serializer for RpcTransactionInclusionIndicesLocator {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(RpcHash, &self.block_hash, writer)?;
        store!(Vec<TransactionIndexType>, &self.indices_within_block, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionInclusionIndicesLocator {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let block_hash = load!(RpcHash, reader)?;
        let indices_within_block = load!(Vec<TransactionIndexType>, reader)?;

        Ok(Self { block_hash, indices_within_block })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct RpcTransactionAcceptingDaaScoreLocator {
    pub accepting_daa_score: u64,
    pub transaction_ids: Vec<RpcTransactionId>,
}

impl RpcTransactionAcceptingDaaScoreLocator {
    pub fn new(accepting_daa_score: u64, transaction_ids: Vec<RpcTransactionId>) -> Self {
        Self { accepting_daa_score, transaction_ids }
    }
}

impl Serializer for RpcTransactionAcceptingDaaScoreLocator {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(u64, &self.accepting_daa_score, writer)?;
        store!(Vec<RpcTransactionId>, &self.transaction_ids, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionAcceptingDaaScoreLocator {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let accepting_daa_score = load!(u64, reader)?;
        let transaction_ids = load!(Vec<RpcTransactionId>, reader)?;

        Ok(Self { accepting_daa_score, transaction_ids })
    }
}

/// Verbosity switches
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionOutpointVerbosity {
    pub include_transaction_id: Option<bool>,
    pub include_index: Option<bool>,
}

impl RpcTransactionOutpointVerbosity {
    pub fn new(include_transaction_id: Option<bool>, include_index: Option<bool>) -> Self {
        Self { include_transaction_id, include_index }
    }
}

impl Serializer for RpcTransactionOutpointVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(Option<bool>, &self.include_transaction_id, writer)?;
        store!(Option<bool>, &self.include_index, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionOutpointVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let include_transaction_id = load!(Option<bool>, reader)?;
        let include_index = load!(Option<bool>, reader)?;

        Ok(Self { include_transaction_id, include_index })
    }
}

// RpcUtxoEntryVerbosity
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcUtxoEntryVerbosity {
    pub include_amount: Option<bool>,
    pub include_script_public_key: Option<bool>,
    pub include_block_daa_score: Option<bool>,
    pub include_is_coinbase: Option<bool>,
}

impl RpcUtxoEntryVerbosity {
    pub fn new(
        include_amount: Option<bool>,
        include_script_public_key: Option<bool>,
        include_block_daa_score: Option<bool>,
        include_is_coinbase: Option<bool>,
    ) -> Self {
        Self { include_amount, include_script_public_key, include_block_daa_score, include_is_coinbase }
    }
}

impl Serializer for RpcUtxoEntryVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(Option<bool>, &self.include_amount, writer)?;
        store!(Option<bool>, &self.include_script_public_key, writer)?;
        store!(Option<bool>, &self.include_block_daa_score, writer)?;
        store!(Option<bool>, &self.include_is_coinbase, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcUtxoEntryVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;

        let include_amount = load!(Option<bool>, reader)?;
        let include_script_public_key = load!(Option<bool>, reader)?;
        let include_block_daa_score = load!(Option<bool>, reader)?;
        let include_is_coinbase = load!(Option<bool>, reader)?;

        Ok(Self { include_amount, include_script_public_key, include_block_daa_score, include_is_coinbase })
    }
}

// RpcTransactionInputVerbosity
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionInputVerbosity {
    pub include_previous_outpoint: Option<bool>,
    pub include_signature_script: Option<bool>,
    pub include_sequence: Option<bool>,
    pub include_sig_op_count: Option<bool>,
    pub verbose_data_verbosity: Option<RpcTransactionInputVerboseDataVerbosity>,
}

impl RpcTransactionInputVerbosity {
    pub fn new(
        include_previous_outpoint: Option<bool>,
        include_signature_script: Option<bool>,
        include_sequence: Option<bool>,
        include_sig_op_count: Option<bool>,
        verbose_data_verbosity: Option<RpcTransactionInputVerboseDataVerbosity>,
    ) -> Self {
        Self { include_previous_outpoint, include_signature_script, include_sequence, include_sig_op_count, verbose_data_verbosity }
    }
}

impl Serializer for RpcTransactionInputVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(Option<bool>, &self.include_previous_outpoint, writer)?;
        store!(Option<bool>, &self.include_signature_script, writer)?;
        store!(Option<bool>, &self.include_sequence, writer)?;
        store!(Option<bool>, &self.include_sig_op_count, writer)?;
        serialize!(Option<RpcTransactionInputVerboseDataVerbosity>, &self.verbose_data_verbosity, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionInputVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;

        let include_previous_outpoint = load!(Option<bool>, reader)?;
        let include_signature_script = load!(Option<bool>, reader)?;
        let include_sequence = load!(Option<bool>, reader)?;
        let include_sig_op_count = load!(Option<bool>, reader)?;
        let verbose_data_verbosity = deserialize!(Option<RpcTransactionInputVerboseDataVerbosity>, reader)?;

        Ok(Self {
            include_previous_outpoint,
            include_signature_script,
            include_sequence,
            include_sig_op_count,
            verbose_data_verbosity,
        })
    }
}

// RpcTransactionInputVerboseDataVerbosity
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionInputVerboseDataVerbosity {
    pub utxo_entry_verbosity: Option<RpcUtxoEntryVerbosity>,
}

impl RpcTransactionInputVerboseDataVerbosity {
    pub fn new(utxo_entry_verbosity: Option<RpcUtxoEntryVerbosity>) -> Self {
        Self { utxo_entry_verbosity }
    }
}

impl Serializer for RpcTransactionInputVerboseDataVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        serialize!(Option<RpcUtxoEntryVerbosity>, &self.utxo_entry_verbosity, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionInputVerboseDataVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;
        let utxo_entry_verbosity = deserialize!(Option<RpcUtxoEntryVerbosity>, reader)?;

        Ok(Self { utxo_entry_verbosity })
    }
}

// RpcTransactionOutputVerbosity

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionOutputVerbosity {
    pub include_amount: Option<bool>,
    pub include_script_public_key: Option<bool>,
    pub verbose_data_verbosity: Option<RpcTransactionOutputVerboseDataVerbosity>,
}

impl RpcTransactionOutputVerbosity {
    pub fn new(
        include_amount: Option<bool>,
        include_script_public_key: Option<bool>,
        verbose_data_verbosity: Option<RpcTransactionOutputVerboseDataVerbosity>,
    ) -> Self {
        Self { include_amount, include_script_public_key, verbose_data_verbosity }
    }
}

impl Serializer for RpcTransactionOutputVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(Option<bool>, &self.include_amount, writer)?;
        store!(Option<bool>, &self.include_script_public_key, writer)?;
        serialize!(Option<RpcTransactionOutputVerboseDataVerbosity>, &self.verbose_data_verbosity, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionOutputVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;

        let include_amount = load!(Option<bool>, reader)?;
        let include_script_public_key = load!(Option<bool>, reader)?;
        let verbose_data_verbosity = deserialize!(Option<RpcTransactionOutputVerboseDataVerbosity>, reader)?;

        Ok(Self { include_amount, include_script_public_key, verbose_data_verbosity })
    }
}

// RpcTransactionOutputVerboseDataVerbosity
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionOutputVerboseDataVerbosity {
    pub include_script_public_key_type: Option<bool>,
    pub include_script_public_key_address: Option<bool>,
}

impl RpcTransactionOutputVerboseDataVerbosity {
    pub fn new(include_script_public_key_type: Option<bool>, include_script_public_key_address: Option<bool>) -> Self {
        Self { include_script_public_key_type, include_script_public_key_address }
    }
}

impl Serializer for RpcTransactionOutputVerboseDataVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(Option<bool>, &self.include_script_public_key_type, writer)?;
        store!(Option<bool>, &self.include_script_public_key_address, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionOutputVerboseDataVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;

        let include_script_public_key_type = load!(Option<bool>, reader)?;
        let include_script_public_key_address = load!(Option<bool>, reader)?;

        Ok(Self { include_script_public_key_type, include_script_public_key_address })
    }
}

// RpcTransactionVerbosity
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionVerbosity {
    pub include_version: Option<bool>,
    pub input_verbosity: Option<RpcTransactionInputVerbosity>,
    pub output_verbosity: Option<RpcTransactionOutputVerbosity>,
    pub include_lock_time: Option<bool>,
    pub include_subnetwork_id: Option<bool>,
    pub include_gas: Option<bool>,
    pub include_payload: Option<bool>,
    pub include_mass: Option<bool>,
    pub verbose_data_verbosity: Option<RpcTransactionVerboseDataVerbosity>,
}

impl RpcTransactionVerbosity {
    pub fn new(
        include_version: Option<bool>,
        input_verbosity: Option<RpcTransactionInputVerbosity>,
        output_verbosity: Option<RpcTransactionOutputVerbosity>,
        include_lock_time: Option<bool>,
        include_subnetwork_id: Option<bool>,
        include_gas: Option<bool>,
        include_payload: Option<bool>,
        include_mass: Option<bool>,
        verbose_data_verbosity: Option<RpcTransactionVerboseDataVerbosity>,
    ) -> Self {
        Self {
            include_version,
            input_verbosity,
            output_verbosity,
            include_lock_time,
            include_subnetwork_id,
            include_gas,
            include_payload,
            include_mass,
            verbose_data_verbosity,
        }
    }

    pub fn requires_populated_transaction(&self) -> bool {
        self.input_verbosity
            .as_ref()
            .is_some_and(|active| active.verbose_data_verbosity.as_ref().is_some_and(|active| active.utxo_entry_verbosity.is_some()))
    }

    pub fn requires_block_hash(&self) -> bool {
        self.verbose_data_verbosity.as_ref().is_some_and(|active| active.include_block_hash.is_some())
    }

    pub fn requires_block_time(&self) -> bool {
        self.verbose_data_verbosity.as_ref().is_some_and(|active| active.include_block_time.is_some())
    }
}

impl Serializer for RpcTransactionVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(Option<bool>, &self.include_version, writer)?;
        serialize!(Option<RpcTransactionInputVerbosity>, &self.input_verbosity, writer)?;
        serialize!(Option<RpcTransactionOutputVerbosity>, &self.output_verbosity, writer)?;
        store!(Option<bool>, &self.include_lock_time, writer)?;
        store!(Option<bool>, &self.include_subnetwork_id, writer)?;
        store!(Option<bool>, &self.include_gas, writer)?;
        store!(Option<bool>, &self.include_payload, writer)?;
        store!(Option<bool>, &self.include_mass, writer)?;
        serialize!(Option<RpcTransactionVerboseDataVerbosity>, &self.verbose_data_verbosity, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;

        let include_version = load!(Option<bool>, reader)?;
        let input_verbosity = deserialize!(Option<RpcTransactionInputVerbosity>, reader)?;
        let output_verbosity = deserialize!(Option<RpcTransactionOutputVerbosity>, reader)?;
        let include_lock_time = load!(Option<bool>, reader)?;
        let include_subnetwork_id = load!(Option<bool>, reader)?;
        let include_gas = load!(Option<bool>, reader)?;
        let include_payload = load!(Option<bool>, reader)?;
        let include_mass = load!(Option<bool>, reader)?;
        let verbose_data_verbosity = deserialize!(Option<RpcTransactionVerboseDataVerbosity>, reader)?;

        Ok(Self {
            include_version,
            input_verbosity,
            output_verbosity,
            include_lock_time,
            include_subnetwork_id,
            include_gas,
            include_payload,
            include_mass,
            verbose_data_verbosity,
        })
    }
}

// RpcTransactionVerboseDataVerbosity
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionVerboseDataVerbosity {
    pub include_transaction_id: Option<bool>,
    pub include_hash: Option<bool>,
    pub include_compute_mass: Option<bool>,
    pub include_block_hash: Option<bool>,
    pub include_block_time: Option<bool>,
}

impl RpcTransactionVerboseDataVerbosity {
    pub fn new(
        include_transaction_id: Option<bool>,
        include_hash: Option<bool>,
        include_compute_mass: Option<bool>,
        include_block_hash: Option<bool>,
        include_block_time: Option<bool>,
    ) -> Self {
        Self { include_transaction_id, include_hash, include_compute_mass, include_block_hash, include_block_time }
    }
}

impl Serializer for RpcTransactionVerboseDataVerbosity {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u8, &1, writer)?;
        store!(Option<bool>, &self.include_transaction_id, writer)?;
        store!(Option<bool>, &self.include_hash, writer)?;
        store!(Option<bool>, &self.include_compute_mass, writer)?;
        store!(Option<bool>, &self.include_block_hash, writer)?;
        store!(Option<bool>, &self.include_block_time, writer)?;

        Ok(())
    }
}

impl Deserializer for RpcTransactionVerboseDataVerbosity {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u8, reader)?;

        let include_transaction_id = load!(Option<bool>, reader)?;
        let include_hash = load!(Option<bool>, reader)?;
        let include_compute_mass = load!(Option<bool>, reader)?;
        let include_block_hash = load!(Option<bool>, reader)?;
        let include_block_time = load!(Option<bool>, reader)?;

        Ok(Self { include_transaction_id, include_hash, include_compute_mass, include_block_hash, include_block_time })
    }
}
