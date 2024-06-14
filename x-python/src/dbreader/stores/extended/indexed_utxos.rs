use csv::Writer;
use itertools::Itertools;
use kaspa_utxoindex::stores::indexed_utxos::{
    DbUtxoSetByScriptPublicKeyStore,
    ScriptPublicKeyBucket,
    UtxoEntryFullAccessKey,
    TRANSACTION_OUTPOINT_KEY_SIZE
};
use kaspa_addresses::Prefix;
use kaspa_consensus_core::tx::ScriptPublicKey;
use kaspa_txscript::extract_script_pub_key_address;
use kaspa_utxoindex::model::CompactUtxoEntry;
use std::{collections::HashMap, fs::File, path::PathBuf, sync::Arc};

pub trait UtxoEntryFullAccessKeyExt {
    fn extract_script_public_key_bucket(&self) -> ScriptPublicKeyBucket;
}

impl UtxoEntryFullAccessKeyExt for UtxoEntryFullAccessKey {
    fn extract_script_public_key_bucket(&self) -> ScriptPublicKeyBucket {
        // Extract the portion of the bytes that corresponds to the ScriptPublicKeyBucket
        let script_public_key_bucket_end = self.0.len() - TRANSACTION_OUTPOINT_KEY_SIZE;
        ScriptPublicKeyBucket(self.0[..script_public_key_bucket_end].to_vec())
    }
}

pub trait UtxoByScriptPublicKeyStoreExt {
    fn export_all_outpoints(&self, filepath: String, address: bool, daa_score: bool, amount: bool, is_coinbase: bool, outpoint: bool, chunk_size: i32, verbose: bool) -> i64; 
    fn export_all_addresses(&self, filepath: String, chunk_size: i32, verbose: bool) -> i64;
}

impl UtxoByScriptPublicKeyStoreExt for DbUtxoSetByScriptPublicKeyStore {
    // Export all outpoints to a local CSV file
    fn export_all_outpoints(
        &self,
        filepath: String,
        address: bool,
        daa_score: bool,
        amount: bool,
        is_coinbase: bool,
        outpoint: bool,
        chunk_size: i32,
        verbose: bool,
    ) -> i64 {

        // Create CSV file and writer
        let final_path = PathBuf::from(filepath);
        let file = File::create(&final_path).expect("Failed to create file");
        let mut wtr = Writer::from_writer(file); // TODO outdir

        // Write CSV headers row
        let mut headers = Vec::new();
        if address {
            headers.push("address");
        }
        if daa_score {
            headers.push("daa_score");
        }
        if amount {
            headers.push("amount");
        }
        if is_coinbase {
            headers.push("is_coinbase");
        }
        if outpoint {
            headers.push("transaction_id");
            headers.push("transaction_index");
        }
        let _ = wtr.write_record(&headers);

        let mut chunk_count = 0;
        let mut utxo_count: i64 = 0;

        let iter = self.access.iterator();
        for chunk in &iter.chunks(chunk_size.try_into().unwrap()) {
            for r in chunk {
                let (key, value): (Box<[u8]>, CompactUtxoEntry) = r.unwrap();

                // Convert key to UtxoEntryFullAccessKey
                let utxo_entry_full_access_key = UtxoEntryFullAccessKey(Arc::new(key.to_vec()));

                // Build row for CSV file
                let mut row: Vec<String> = Vec::new();
                if address {
                    // Extract ScriptPublicKeyBucket from UtxoEntryFullAccessKey
                    let script_public_key_bucket = utxo_entry_full_access_key.extract_script_public_key_bucket();

                    // Get ScriptPublicKey from ScriptPublicKeyBucket
                    let script_public_key = ScriptPublicKey::from(script_public_key_bucket);

                    // Convert ScriptPublicKey to address
                    // TODO use prefix passed on store init
                    let addr = extract_script_pub_key_address(&script_public_key, Prefix::Mainnet).unwrap();

                    // Add to row
                    row.push(addr.to_string());
                }

                if daa_score {
                    row.push(value.block_daa_score.to_string());
                }

                if amount {
                    row.push(value.amount.to_string());
                }

                if is_coinbase {
                    // Convert is_coinbase to single char to reduce file size
                    let coinbase_value = if value.is_coinbase { "t" } else { "f" };
                    row.push(coinbase_value.to_string());
                }

                if outpoint {
                    // Extract TransactionOutpoint from UtxoEntryFullAccessKey
                    let transaction_outpoint = utxo_entry_full_access_key.extract_outpoint();
                    let outpoint_tx_id = transaction_outpoint.transaction_id;
                    let outpoint_tx_idx = transaction_outpoint.index;

                    row.push(outpoint_tx_id.to_string());
                    row.push(outpoint_tx_idx.to_string());
                }

                let _ = wtr.write_record(&row);

                // TODO probably should use chunk but feeling lazy atm
                utxo_count += 1;
            }

            if verbose {
                chunk_count += 1;
                println!("{} UTXOs written to CSV in {} chunks", utxo_count, chunk_count);
            }
        }

        utxo_count
    }

    fn export_all_addresses(&self, filepath: String, chunk_size: i32, verbose: bool) -> i64 {
        let mut addresses: HashMap<String, u64> = HashMap::new();
        
        let mut chunk_count = 0;
        let iter = self.access.iterator();
        for chunk in &iter.chunks(chunk_size.try_into().unwrap()) {
            for r in chunk {
                let (key, value): (Box<[u8]>, CompactUtxoEntry) = r.unwrap();

                let utxo_entry_full_access_key = UtxoEntryFullAccessKey(Arc::new(key.to_vec()));
                let script_public_key_bucket = utxo_entry_full_access_key.extract_script_public_key_bucket();
                let script_public_key = ScriptPublicKey::from(script_public_key_bucket);
                let addr = extract_script_pub_key_address(&script_public_key, Prefix::Mainnet).unwrap();

                // Sum the amounts
                *addresses.entry(addr.to_string()).or_insert(0) += value.amount;
            }
            
            chunk_count += 1;
            if verbose {
                println!("Processed {} utxo entry chunks", chunk_count);
            }
        }

        let final_path = PathBuf::from(filepath);
        let file = File::create(&final_path).expect("Failed to create file");
        let mut wtr = Writer::from_writer(file);

        // Write headers
        let _ = wtr.write_record(&["address", "amount"]);

        // Write summed amounts to CSV
        let mut addr_count = 0;
        for (key, sum) in addresses {
            let _ = wtr.write_record(&[key, sum.to_string()]);
            addr_count += 1;
        }

        addr_count
    }
}