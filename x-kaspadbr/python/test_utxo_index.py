from kaspadbr import DBReader

if __name__ == "__main__":
    db = DBReader(app_dir='/opt2/kaspad-appdir/rust')

    utxo_tips_store = db.stores.utxo_index_tips
    utxo_tips = utxo_tips_store.get()

    # for tip in utxo_tips:
    #     h = db.get_block_header(tip)
    #     print(h['hash'], h['timestamp'], h['daa_score'])

    # Export UTXO set to CSV file
    utxo_index_store = db.stores.utxo_index
    c = utxo_index_store.export(filepath="/opt2/work/dev/test.csv", verbose=True)

    print(f'Exported {c} UTXO records to CSV.')