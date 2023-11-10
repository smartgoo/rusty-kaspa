from kaspadbr import DBReader

if __name__ == "__main__":
    db = DBReader(app_dir='/opt2/kaspad-appdir/rust')

    utxo_tips_store = db.stores.utxo_index_tips
    headers_store = db.stores.headers

    utxo_tips = utxo_tips_store.get()

    bh = headers_store.get(block_hash=utxo_tips[0])

    print(bh)