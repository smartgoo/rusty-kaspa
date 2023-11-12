from kaspadbr import DBReader

if __name__ == "__main__":
    db = DBReader(app_dir='/opt2/kaspad-appdir/rust')

    stores = db.stores

    utxo_tips = stores.utxo_index_tips.get()

    bh = stores.headers.get(block_hash=utxo_tips[0])

    print(bh)