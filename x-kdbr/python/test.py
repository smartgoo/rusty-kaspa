from kdbr import Reader

if __name__ == "__main__":
    store = Reader(app_dir='/opt2/kaspad-appdir/rust')

    print(store.home_dir)
    print(store.app_dir)
    print(store.network_dir)
    print(store.db_dir)
    print(store.utxo_index_db_dir)
    print(store.meta_db_dir)
    print(store.consensus_db_dir)

    print(store.get_cs() / 100_000_000)
    print(store.get_utxo_tips())

    print(store.export_all_outpoints())