from kaspapy import DBReader

if __name__ == "__main__":
    db = DBReader(app_dir='/Users/chuckievallone/.rusty-kaspa')

    stores = db.stores
    utxo_tips = stores.utxo_index_tips.get()

    print(utxo_tips)