from kaspadbr import DBReader

if __name__ == "__main__":
    db = DBReader(app_dir='/opt2/kaspad-appdir/rust')

    stores = db.stores
    utxo_tips = stores.utxo_index_tips.get()

    for tip_hash in utxo_tips:
        h = stores.headers.get(tip_hash)
        print(h['hash'], h['timestamp'], h['daa_score'])

    # Export UTXO set to CSV file
    # c = stores.utxo_index.export(filepath="/opt2/work/dev/test.csv", verbose=True)
    # print(f'Exported {c} UTXO records to CSV.')

    # Export addresses to CSV file
    c = stores.utxo_index.export_addresses(filepath="/opt2/work/dev/addrs.csv", verbose=True)
    print(c)
