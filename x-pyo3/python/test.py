from rusty_kaspa_db import Reader

if __name__ == "__main__":
    store = Reader(app_dir='/opt2/kaspad-appdr/rust')

    print(store.home_dir)
    print(store.app_dir)
    print(store.network_dir)
    print(store.db_dir)
    print(store.utxo_index_db_dir)
    print(store.meta_db_dir)
    print(store.consensus_db_dir)