// use crate::core::model::{CompactUtxoCollection, CompactUtxoEntry, UtxoSetByScriptPublicKey};

use chrono::prelude::*;
use csv::Writer;
use itertools::Itertools;
use kaspa_addresses::Prefix;
use kaspa_consensus_core::tx::{
    ScriptPublicKey, ScriptPublicKeyVersion, ScriptPublicKeys, ScriptVec, TransactionIndexType, TransactionOutpoint,
};
use kaspa_database::prelude::{CachedDbAccess, StoreResult, DB};
use kaspa_database::registry::DatabaseStorePrefixes;
use kaspa_hashes::Hash;
use kaspa_utxoindex::model::{CompactUtxoCollection, CompactUtxoEntry, UtxoSetByScriptPublicKey};
use kaspa_txscript::extract_script_pub_key_address;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
// use std::error::Error;
use std::{
    fmt::Display,
    fs::File,
    mem::size_of,
    path::{Path, PathBuf},
    sync::Arc,
};

pub const VERSION_TYPE_SIZE: usize = size_of::<ScriptPublicKeyVersion>(); // Const since we need to re-use this a few times.

/// [`ScriptPublicKeyBucket`].
/// Consists of 2 bytes of little endian [VersionType] bytes, followed by a variable size of [ScriptVec].
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct ScriptPublicKeyBucket(Vec<u8>);

impl From<&ScriptPublicKey> for ScriptPublicKeyBucket {
    fn from(script_public_key: &ScriptPublicKey) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(VERSION_TYPE_SIZE + script_public_key.script().len());
        bytes.extend_from_slice(&script_public_key.version().to_le_bytes());
        bytes.extend_from_slice(&(script_public_key.script().len() as u64).to_le_bytes()); // TODO: Consider using a smaller integer
        bytes.extend_from_slice(script_public_key.script());
        Self(bytes)
    }
}

impl From<ScriptPublicKeyBucket> for ScriptPublicKey {
    fn from(bucket: ScriptPublicKeyBucket) -> Self {
        let version = ScriptPublicKeyVersion::from_le_bytes(
            <[u8; VERSION_TYPE_SIZE]>::try_from(&bucket.0[..VERSION_TYPE_SIZE]).expect("expected version size"),
        );

        let script_size =
            u64::from_le_bytes(bucket.0[VERSION_TYPE_SIZE..VERSION_TYPE_SIZE + size_of::<u64>()].try_into().unwrap()) as usize;
        let script =
            ScriptVec::from_slice(&bucket.0[VERSION_TYPE_SIZE + size_of::<u64>()..VERSION_TYPE_SIZE + size_of::<u64>() + script_size]);

        Self::new(version, script)
    }
}

impl AsRef<[u8]> for ScriptPublicKeyBucket {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

// Keys:

// TransactionOutpoint:
/// Size of the [TransactionOutpointKey] in bytes.
pub const TRANSACTION_OUTPOINT_KEY_SIZE: usize = kaspa_hashes::HASH_SIZE + size_of::<TransactionIndexType>();

/// [TransactionOutpoint] key which references the [CompactUtxoEntry] within a [ScriptPublicKeyBucket]
/// Consists of 32 bytes of [TransactionId], followed by 4 bytes of little endian [TransactionIndexType]
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct TransactionOutpointKey([u8; TRANSACTION_OUTPOINT_KEY_SIZE]);

impl From<TransactionOutpointKey> for TransactionOutpoint {
    fn from(key: TransactionOutpointKey) -> Self {
        let transaction_id = Hash::from_slice(&key.0[..kaspa_hashes::HASH_SIZE]);
        let index = TransactionIndexType::from_le_bytes(
            <[u8; std::mem::size_of::<TransactionIndexType>()]>::try_from(&key.0[kaspa_hashes::HASH_SIZE..])
                .expect("expected index size"),
        );
        Self::new(transaction_id, index)
    }
}

impl From<&TransactionOutpoint> for TransactionOutpointKey {
    fn from(outpoint: &TransactionOutpoint) -> Self {
        let mut bytes = [0; TRANSACTION_OUTPOINT_KEY_SIZE];
        bytes[..kaspa_hashes::HASH_SIZE].copy_from_slice(&outpoint.transaction_id.as_bytes());
        bytes[kaspa_hashes::HASH_SIZE..].copy_from_slice(&outpoint.index.to_le_bytes());
        Self(bytes)
    }
}

impl AsRef<[u8]> for TransactionOutpointKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Full [CompactUtxoEntry] access key.
/// Consists of variable amount of bytes of [ScriptPublicKeyBucket], and 36 bytes of [TransactionOutpointKey]
#[derive(Eq, Hash, PartialEq, Debug, Clone, Serialize, Deserialize)]
struct UtxoEntryFullAccessKey(Arc<Vec<u8>>);

impl Display for UtxoEntryFullAccessKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // TODO: Deserialize first
    }
}

impl UtxoEntryFullAccessKey {
    pub fn extract_outpoint(&self) -> TransactionOutpoint {
        TransactionOutpoint::from(TransactionOutpointKey(self.0[(self.0.len() - TRANSACTION_OUTPOINT_KEY_SIZE)..].try_into().unwrap()))
    }

    pub fn extract_script_public_key_bucket(&self) -> ScriptPublicKeyBucket {
        // Extract the portion of the bytes that corresponds to the ScriptPublicKeyBucket
        let script_public_key_bucket_end = self.0.len() - TRANSACTION_OUTPOINT_KEY_SIZE;
        ScriptPublicKeyBucket(self.0[..script_public_key_bucket_end].to_vec())
    }
}

impl AsRef<[u8]> for UtxoEntryFullAccessKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

// Traits:

pub trait UtxoSetByScriptPublicKeyStoreReader {
    /// Get [UtxoSetByScriptPublicKey] set by queried [ScriptPublicKeys],
    fn get_utxos_from_script_public_keys(&self, script_public_keys: ScriptPublicKeys) -> StoreResult<UtxoSetByScriptPublicKey>;
    fn get_all_outpoints(&self) -> StoreResult<HashSet<TransactionOutpoint>>; // This can have a big memory footprint, so it should be used only for tests.
    fn export_all_outpoints(&self) -> PathBuf;
}

// Implementations:

#[derive(Clone)]
pub struct DbUtxoSetByScriptPublicKeyStore {
    db: Arc<DB>,
    access: CachedDbAccess<UtxoEntryFullAccessKey, CompactUtxoEntry>,
}

impl DbUtxoSetByScriptPublicKeyStore {
    pub fn new(db: Arc<DB>, cache_size: u64) -> Self {
        Self { db: Arc::clone(&db), access: CachedDbAccess::new(db, cache_size, DatabaseStorePrefixes::UtxoIndex.into()) }
    }
}

impl DbUtxoSetByScriptPublicKeyStore {
    // compared to go-kaspad this gets transaction outpoints from multiple script public keys at once.
    // TODO: probably ideal way to retrieve is to return a chained iterator which can be used to chunk results and propagate utxo entries
    // to the rpc via pagination, this would alleviate the memory footprint of script public keys with large amount of utxos.
    fn get_utxos_from_script_public_keys(&self, script_public_keys: ScriptPublicKeys) -> StoreResult<UtxoSetByScriptPublicKey> {
        let mut utxos_by_script_public_keys = UtxoSetByScriptPublicKey::new();
        for script_public_key in script_public_keys.into_iter() {
            let script_public_key_bucket = ScriptPublicKeyBucket::from(&script_public_key);
            let utxos_by_script_public_keys_inner = CompactUtxoCollection::from_iter(
                self.access.seek_iterator(Some(script_public_key_bucket.as_ref()), None, usize::MAX, false).map(|res| {
                    let (key, entry) = res.unwrap();
                    (TransactionOutpointKey(<[u8; TRANSACTION_OUTPOINT_KEY_SIZE]>::try_from(&key[..]).unwrap()).into(), entry)
                }),
            );
            utxos_by_script_public_keys.insert(script_public_key, utxos_by_script_public_keys_inner);
        }
        Ok(utxos_by_script_public_keys)
    }

    // This can have a big memory footprint, so it should be used only for tests.
    fn get_all_outpoints(&self) -> StoreResult<HashSet<TransactionOutpoint>> {
        Ok(HashSet::from_iter(
            self.access.iterator().map(|res| UtxoEntryFullAccessKey(Arc::new(res.unwrap().0.to_vec())).extract_outpoint()),
        ))
    }

    pub fn export_all_outpoints(
        &self,
        filepath: String,
        chunk_size: i32,
        verbose: bool,
        address: bool,
        script_public_key: bool,
        daa_score: bool,
        amount: bool,
        is_coinbase: bool
    ) -> i64 { // TODO what to return?

        // Create CSV file and writer
        let final_path = PathBuf::from(filepath);
        let file = File::create(&final_path).expect("Failed to create file");
        let mut wtr = Writer::from_writer(file); // TODO outdir

        // Write CSV headers row
        let mut headers = Vec::new();
        if address {
            headers.push("address");
        }
        // if script_public_key { // TODO
        //     headers.push("script_public_key");
        // }
        if daa_score {
            headers.push("daa_score");
        }
        if amount {
            headers.push("amount");
        }
        if is_coinbase {
            headers.push("is_coinbase");
        }
        let _ = wtr.write_record(&headers);

        // let chunk_size = 100_000;
        let mut chunk_count = 0;
        let mut utxo_count: i64 = 0;

        let iter = self.access.iterator();
        for chunk in &iter.chunks(chunk_size.try_into().unwrap()) {
            for r in chunk {
                // TODO optimize. Lots of opportunities in this chunk of code to do so 

                let (key, value) = r.unwrap();

                let mut row = Vec::new();
                
                // Convert key to UtxoEntryFullAccessKey 
                let utxo_entry_full_access_key = UtxoEntryFullAccessKey(Arc::new(key.to_vec()));

                // TODO also extract TransactionOutpointKey and include optionally based on args

                // Pull ScriptPublicKeyBucket out of UtxoEntryFullAccessKey
                let script_public_key_bucket = utxo_entry_full_access_key.extract_script_public_key_bucket();

                // Pull ScriptPublicKey out of ScriptPublicKeyBucket
                let script_public_key = ScriptPublicKey::from(script_public_key_bucket);
                // if {script_public_key}

                // Convert to address
                let addr = extract_script_pub_key_address(&script_public_key, Prefix::Mainnet).unwrap();

                // Add to row according to params
                if address {
                    row.push(addr.to_string());
                }
                // if script_public_key { // TODO
                //     headers.push("script_public_key");
                // }
                if daa_score {
                    row.push(value.block_daa_score.to_string());
                }
                if amount {
                    row.push(value.amount.to_string());
                }
                if is_coinbase {
                    let coinbase_value = if value.is_coinbase { "t" } else { "f" };
                    row.push(coinbase_value.to_string());
                }

                // Write to CSV. value is type CompactUtxoEntry
                let _ = wtr.write_record(&row);

                utxo_count += 1; // TODO probably should use chunk but being lazy atm
            }

            if verbose {
                chunk_count += 1;
                println!("{} chunks written to csv", chunk_count);
            }
        }

        utxo_count
    }

}
