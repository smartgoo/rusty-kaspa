from kaspadbr import DBReader

if __name__ == "__main__":
    db = DBReader(app_dir='/opt2/kaspad-appdir/rust')

    utxo_tips_store = db.stores.utxo_index_tips
    utxo_tips = utxo_tips_store.get()
    print(utxo_tips)