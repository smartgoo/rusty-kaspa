use async_trait::async_trait;
use kaspa_addresses::{Address, AddressError};
use kaspa_consensus_core::{
    acceptance_data::MergesetBlockAcceptanceData, block::Block, config::Config, hashing::tx::hash, header::Header, tx::{MutableTransaction, Transaction, TransactionId, TransactionInput, TransactionOutput, TransactionQueryResult, TransactionType, UtxoEntry}, ChainPath
};
use kaspa_consensus_notify::notification::{self as consensus_notify, Notification as ConsensusNotification};
use kaspa_consensusmanager::{ConsensusManager, ConsensusProxy};
use kaspa_hashes::Hash;
use kaspa_math::Uint256;
use kaspa_mining::model::{owner_txs::OwnerTransactions, TransactionIdSet};
use kaspa_notify::converter::Converter;
use kaspa_rpc_core::{
    BlockAddedNotification, Notification, RpcAcceptanceData, RpcAcceptedTransactionIds, RpcBlock, RpcBlockVerboseData, RpcHash, RpcHeader, RpcMempoolEntry, RpcMempoolEntryByAddress, RpcMergesetBlockAcceptanceData, RpcResult, RpcTransaction, RpcTransactionInput, RpcTransactionInputVerboseData, RpcTransactionOutput, RpcTransactionOutputVerboseData, RpcTransactionVerboseData, RpcUtxoEntry, RpcUtxoEntryVerboseData
};
use kaspa_txscript::{extract_script_pub_key_address, script_class::ScriptClass};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

/// Conversion of consensus_core to rpc_core structures
pub struct ConsensusConverter {
    consensus_manager: Arc<ConsensusManager>,
    config: Arc<Config>,
}

impl ConsensusConverter {
    pub fn new(consensus_manager: Arc<ConsensusManager>, config: Arc<Config>) -> Self {
        Self { consensus_manager, config }
    }

    /// Returns the proof-of-work difficulty as a multiple of the minimum difficulty using
    /// the passed bits field from the header of a block.
    pub fn get_difficulty_ratio(&self, bits: u32) -> f64 {
        // The minimum difficulty is the max possible proof-of-work limit bits
        // converted back to a number. Note this is not the same as the proof of
        // work limit directly because the block difficulty is encoded in a block
        // with the compact form which loses precision.
        let target = Uint256::from_compact_target_bits(bits);
        self.config.max_difficulty_target_f64 / target.as_f64()
    }

    /// Converts a consensus [`Block`] into an [`RpcBlock`], optionally including transaction verbose data.
    ///
    /// _GO-KASPAD: PopulateBlockWithVerboseData_
    pub async fn get_block(
        &self,
        consensus: &ConsensusProxy,
        block: &Block,
        include_transactions: bool,
        include_transaction_verbose_data: bool,
    ) -> RpcResult<RpcBlock> {
        let hash = block.hash();
        let ghostdag_data = consensus.async_get_ghostdag_data(hash).await?;
        let block_status = consensus.async_get_block_status(hash).await.unwrap();
        let children = consensus.async_get_block_children(hash).await.unwrap_or_default();
        let is_chain_block = consensus.async_is_chain_block(hash).await?;
        let verbose_data = Some(RpcBlockVerboseData {
            hash,
            difficulty: self.get_difficulty_ratio(block.header.bits),
            selected_parent_hash: ghostdag_data.selected_parent,
            transaction_ids: block.transactions.iter().map(|x| x.id()).collect(),
            is_header_only: block_status.is_header_only(),
            blue_score: ghostdag_data.blue_score,
            children_hashes: children,
            merge_set_blues_hashes: ghostdag_data.mergeset_blues,
            merge_set_reds_hashes: ghostdag_data.mergeset_reds,
            is_chain_block,
        });

        let transactions = if include_transactions {
            block
                .transactions
                .iter()
                .map(|x| self.get_transaction(consensus, x, Some(&block.header), include_transaction_verbose_data))
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        Ok(RpcBlock { header: block.header.as_ref().into(), transactions, verbose_data })
    }

    pub fn get_mempool_entry(&self, consensus: &ConsensusProxy, transaction: &MutableTransaction) -> RpcMempoolEntry {
        let is_orphan = !transaction.is_fully_populated();
        let rpc_transaction = self.get_transaction(consensus, &transaction.tx, None, true);
        RpcMempoolEntry::new(transaction.calculated_fee.unwrap_or_default(), rpc_transaction, is_orphan)
    }

    pub fn get_mempool_entries_by_address(
        &self,
        consensus: &ConsensusProxy,
        address: Address,
        owner_transactions: &OwnerTransactions,
        transactions: &HashMap<TransactionId, MutableTransaction>,
    ) -> RpcMempoolEntryByAddress {
        let sending = self.get_owner_entries(consensus, &owner_transactions.sending_txs, transactions);
        let receiving = self.get_owner_entries(consensus, &owner_transactions.receiving_txs, transactions);
        RpcMempoolEntryByAddress::new(address, sending, receiving)
    }

    pub fn get_owner_entries(
        &self,
        consensus: &ConsensusProxy,
        transaction_ids: &TransactionIdSet,
        transactions: &HashMap<TransactionId, MutableTransaction>,
    ) -> Vec<RpcMempoolEntry> {
        transaction_ids.iter().map(|x| self.get_mempool_entry(consensus, transactions.get(x).expect("transaction exists"))).collect()
    }

    /// Converts a consensus [`Transaction`] into an [`RpcTransaction`], optionally including verbose data.
    ///
    /// _GO-KASPAD: PopulateTransactionWithVerboseData
    pub fn get_transaction(
        &self,
        consensus: &ConsensusProxy,
        transaction: &Transaction,
        header: Option<&Header>,
        include_verbose_data: bool,
    ) -> RpcTransaction {
        if include_verbose_data {
            let verbose_data = Some(RpcTransactionVerboseData {
                transaction_id: transaction.id(),
                hash: hash(transaction, false),
                compute_mass: consensus.calculate_transaction_non_contextual_masses(transaction).compute_mass,
                // TODO: make block_hash an option
                block_hash: header.map_or_else(RpcHash::default, |x| x.hash),
                block_time: header.map_or(0, |x| x.timestamp),
            });
            RpcTransaction {
                version: transaction.version,
                inputs: transaction.inputs.iter().map(|x| self.get_transaction_input(x)).collect(),
                outputs: transaction.outputs.iter().map(|x| self.get_transaction_output(x)).collect(),
                lock_time: transaction.lock_time,
                subnetwork_id: transaction.subnetwork_id.clone(),
                gas: transaction.gas,
                payload: transaction.payload.clone(),
                mass: transaction.mass(),
                verbose_data,
            }
        } else {
            transaction.into()
        }
    }

    fn get_transaction_input(&self, input: &TransactionInput) -> RpcTransactionInput {
        input.into()
    }

    fn get_transaction_output(&self, output: &TransactionOutput) -> RpcTransactionOutput {
        let script_public_key_type = ScriptClass::from_script(&output.script_public_key);
        let address = extract_script_pub_key_address(&output.script_public_key, self.config.prefix()).ok();
        let verbose_data =
            address.map(|address| RpcTransactionOutputVerboseData { script_public_key_type, script_public_key_address: address });
        RpcTransactionOutput { value: output.value, script_public_key: output.script_public_key.clone(), verbose_data }
    }

    pub async fn get_virtual_chain_accepted_transaction_ids(
        &self,
        consensus: &ConsensusProxy,
        chain_path: &ChainPath,
        merged_blocks_limit: Option<usize>,
    ) -> RpcResult<Vec<RpcAcceptedTransactionIds>> {
        let acceptance_data = consensus.async_get_blocks_acceptance_data(chain_path.added.clone(), merged_blocks_limit).await.unwrap();
        Ok(chain_path
            .added
            .iter()
            .zip(acceptance_data.iter())
            .map(|(hash, block_data)| RpcAcceptedTransactionIds {
                accepting_block_hash: hash.to_owned(),
                accepted_transaction_ids: block_data
                    .iter()
                    .flat_map(|x| x.accepted_transactions.iter().map(|tx| tx.transaction_id))
                    .collect(),
            })
            .collect())
    }

    async fn get_header(
        &self,
        consensus: &ConsensusProxy,
        block_hash: RpcHash,
    ) -> RpcResult<RpcHeader> {
        let header = consensus.async_get_header(block_hash).await?;

        Ok(RpcHeader {
            hash: block_hash,
            version: header.version,
            parents_by_level: header.parents_by_level.to_owned(),
            hash_merkle_root: header.hash_merkle_root,
            accepted_id_merkle_root: header.accepted_id_merkle_root,
            utxo_commitment: header.utxo_commitment,
            timestamp: header.timestamp,
            bits: header.bits,
            nonce: header.nonce,
            daa_score: header.daa_score,
            blue_work: header.blue_work,
            blue_score: header.blue_score,
            pruning_point: header.pruning_point,
        })
    }

    fn get_utxo_verbose_data(
        &self,
        utxo: &UtxoEntry,
    ) -> RpcResult<RpcUtxoEntryVerboseData> {
        Ok(RpcUtxoEntryVerboseData {
            script_public_key_type: Some(ScriptClass::from_script(&utxo.script_public_key)),
            script_public_key_address: Some(
                    extract_script_pub_key_address(&utxo.script_public_key, self.config.prefix())
                        .map_err(|_| AddressError::InvalidAddress)?,
                ),
        })
    }

    fn convert_utxo_entry_with_verbosity(&self, utxo: UtxoEntry) -> RpcResult<RpcUtxoEntry> {
        Ok(RpcUtxoEntry {
            amount: utxo.amount,
            script_public_key: utxo.script_public_key.clone(),
            block_daa_score: utxo.block_daa_score,
            is_coinbase: utxo.is_coinbase,
            verbose_data: Some(self.get_utxo_verbose_data(&utxo)?),
        })
    }

    fn get_input_verbose_data(
        &self,
        utxo: Option<UtxoEntry>,
    ) -> RpcResult<RpcTransactionInputVerboseData> {
        Ok(RpcTransactionInputVerboseData {
            utxo_entry: if utxo.is_some() {
                Some(self.convert_utxo_entry_with_verbosity(utxo.unwrap())?)
            } else {
                return Err(kaspa_rpc_core::RpcError::General("UtxoEntry missing".to_string()));
            }
        })
    }

    pub fn get_transaction_input_with_verbose_data(
        &self,
        input: &TransactionInput,
        utxo: Option<UtxoEntry>,
    ) -> RpcResult<RpcTransactionInput> {
        Ok(RpcTransactionInput {
            previous_outpoint: input.previous_outpoint.into(),
            signature_script: input.signature_script.clone(),
            sequence: input.sequence,
            sig_op_count: input.sig_op_count,
            verbose_data: Some(self.get_input_verbose_data(utxo)?),
        })
    }

    fn get_transaction_output_verbose_data(
        &self,
        output: &TransactionOutput,
    ) -> RpcResult<RpcTransactionOutputVerboseData> {
        Ok(RpcTransactionOutputVerboseData {
            script_public_key_type: ScriptClass::from_script(&output.script_public_key),
            script_public_key_address: extract_script_pub_key_address(&output.script_public_key, self.config.prefix())
                    .map_err(|_| AddressError::InvalidAddress)?,
        })
    }

    fn convert_transaction_output(
        &self,
        output: &TransactionOutput,
    ) -> RpcResult<RpcTransactionOutput> {
        Ok(RpcTransactionOutput {
            value: output.value,
            script_public_key: output.script_public_key.clone(),
            verbose_data: Some(self.get_transaction_output_verbose_data(output)?),
        })
    }

    fn get_transaction_verbose_data(
        &self,
        transaction: &Transaction,
        block_hash: Hash,
        block_time: u64,
        compute_mass: u64,
    ) -> RpcResult<RpcTransactionVerboseData> {
        Ok(RpcTransactionVerboseData {
            transaction_id: transaction.id(),
            hash: hash(transaction, true),
            compute_mass: compute_mass,
            block_hash: block_hash,
            block_time: block_time,
        })
    }

    pub async fn convert_transaction(
        &self,
        consensus: &ConsensusProxy,
        transaction: &Transaction,
        block_hash: Option<Hash>,
        block_time: Option<u64>,
    ) -> RpcResult<RpcTransaction> {
        Ok(RpcTransaction {
            version: transaction.version,
            inputs: transaction
                    .inputs
                    .iter()
                    .map(|x| self.get_transaction_input_with_verbose_data(x, None))
                    .collect::<Result<Vec<_>, _>>()?,
            outputs: transaction
                    .outputs
                    .iter()
                    .map(|x| self.convert_transaction_output(x))
                    .collect::<Result<Vec<_>, _>>()?,
            lock_time: transaction.lock_time,
            subnetwork_id: transaction.subnetwork_id.clone(),
            gas: transaction.gas,
            payload: transaction.payload.clone(),
            mass: transaction.mass(),
            verbose_data: {
                let block_time = if let Some(block_time) = block_time {
                    block_time
                } else {
                    consensus.async_get_header(block_hash.unwrap()).await?.timestamp
                };

                Some(self.get_transaction_verbose_data(
                    transaction,
                    block_hash.unwrap(),
                    block_time,
                    consensus.calculate_transaction_non_contextual_masses(transaction).compute_mass,
                )?)
            },
        })
    }

    async fn get_accepted_transactions(
        &self,
        consensus: &ConsensusProxy,
        accepting_chain_block: Hash,
        tx_ids: Option<Vec<TransactionId>>,
        mergeset_block_acceptance: &MergesetBlockAcceptanceData,
        block_time: Option<u64>,
    ) -> RpcResult<Vec<RpcTransaction>> {
        let txs = consensus
            .async_get_transactions_by_accepting_block(
                accepting_chain_block,
                tx_ids,
                TransactionType::Transaction,
            )
            .await?;

        Ok(match txs {
            TransactionQueryResult::Transaction(txs) => {
                let mut converted = Vec::with_capacity(txs.len());

                for tx in txs.iter() {
                    converted.push({
                        let rpc_tx = self
                            .convert_transaction(
                                consensus,
                                tx,
                                Some(mergeset_block_acceptance.block_hash),
                                block_time,
                            )
                            .await?;

                        // if rpc_tx.is_empty() {
                        //     continue;
                        // };

                        rpc_tx
                    });
                }

                converted
            },
            _ => unimplemented!()
        })
    }

    async fn get_mergeset_block_acceptance_data(
        &self,
        consensus: &ConsensusProxy,
        accepting_chain_block: Hash,
        mergeset_block_acceptance: &MergesetBlockAcceptanceData,
    ) -> RpcResult<RpcMergesetBlockAcceptanceData> {
        let merged_header = self.get_header(consensus, mergeset_block_acceptance.block_hash).await?;

        let accepted_transactions = self.get_accepted_transactions(
            consensus,
            accepting_chain_block,
            None,
            mergeset_block_acceptance,
            Some(merged_header.timestamp)
        )
        .await?;

        Ok(RpcMergesetBlockAcceptanceData { merged_header, accepted_transactions })
    }

    pub async fn get_acceptance_data(
        &self,
        consensus: &ConsensusProxy,
        chain_path: &ChainPath,
        merged_blocks_limit: Option<usize>,
    ) -> RpcResult<Vec<RpcAcceptanceData>> {
        let acceptance_data = consensus.async_get_blocks_acceptance_data(chain_path.added.clone(), merged_blocks_limit).await?;
        let mut rpc_acceptance_data = Vec::<RpcAcceptanceData>::with_capacity(acceptance_data.len());

        for (accepting_chain_hash, acceptance_data) in chain_path.added.iter().zip(acceptance_data.iter()) {
            let accepting_chain_header = self.get_header(consensus, *accepting_chain_hash).await?;

            let mut rpc_meregeset_block_acceptance_data = Vec::with_capacity(acceptance_data.len());
            for mergeset_block_acceptance in acceptance_data.iter() {
                rpc_meregeset_block_acceptance_data.push(
                    self.get_mergeset_block_acceptance_data(
                        consensus,
                        *accepting_chain_hash,
                        mergeset_block_acceptance,
                    )
                    .await?,
                );
            }

            rpc_acceptance_data.push(RpcAcceptanceData {
                accepting_chain_block_header: accepting_chain_header,
                mergeset_block_acceptance_data: rpc_meregeset_block_acceptance_data
            });
        }

        Ok(rpc_acceptance_data)
    }
}

#[async_trait]
impl Converter for ConsensusConverter {
    type Incoming = ConsensusNotification;
    type Outgoing = Notification;

    async fn convert(&self, incoming: ConsensusNotification) -> Notification {
        match incoming {
            consensus_notify::Notification::BlockAdded(msg) => {
                let session = self.consensus_manager.consensus().unguarded_session();
                // If get_block fails, rely on the infallible From implementation which will lack verbose data
                let block = Arc::new(self.get_block(&session, &msg.block, true, true).await.unwrap_or_else(|_| (&msg.block).into()));
                Notification::BlockAdded(BlockAddedNotification { block })
            }
            _ => (&incoming).into(),
        }
    }
}

impl Debug for ConsensusConverter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConsensusConverter").field("consensus_manager", &"").field("config", &self.config).finish()
    }
}
