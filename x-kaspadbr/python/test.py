from kaspadbr import Reader

if __name__ == "__main__":
    db = Reader(app_dir='/opt2/kaspad-appdir/rust')

    # Directories
    print('\n')
    print(f'----- DIRECTORIES')
    print(f'Home Dir: {db.home_dir}')
    print(f'App Dir: {db.app_dir}')
    print(f'Network Dir: {db.network_dir}')
    print(f'DB Dir: {db.db_dir}')
    print(f'UTXO DB Dir: {db.utxo_index_db_dir}')
    print(f'Meta DB Dir: {db.meta_db_dir}')
    print(f'Consensus DB Dir: {db.consensus_db_dir}')

    print('\n')
    print(f'----- READER FUNCTIONS')
    print(f'get_current_consensus_entry() -> {db.stores.metadata.current_consensus_key()}')
    print(f'get_circulating_supply() -> {db.stores.circulating_supply.get() / 100_000_000}')

    # Get block that should exist
    # block = store.get_block_header(utxo_tips[0], include_transactions=True)
    # print(f'get_block_header() -> {block["header"].keys()}')

    # # Get block that doesn't exist
    # print(f'get_block_header() -> {store.get_block_header("04a368709289ddfc363c3b0bc4c6db97b1aa4376ebce93f0cd95e6f62e7a7492", include_transactions=True)}')