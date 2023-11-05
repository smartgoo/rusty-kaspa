from kaspadbr import Reader

if __name__ == "__main__":
    store = Reader(app_dir='/opt2/kaspad-appdir/rust')

    # Directories
    print('\n')
    print(f'----- DIRECTORIES')
    print(f'Home Dir: {store.home_dir}')
    print(f'App Dir: {store.app_dir}')
    print(f'Network Dir: {store.network_dir}')
    print(f'DB Dir: {store.db_dir}')
    print(f'UTXO DB Dir: {store.utxo_index_db_dir}')
    print(f'Meta DB Dir: {store.meta_db_dir}')
    print(f'Consensus DB Dir: {store.consensus_db_dir}')

    print('\n')
    print(f'----- READER FUNCTIONS')
    print(f'get_current_consensus_entry() -> {store.get_current_consensus_entry()}')
    # print(f'get_circulating_supply() -> {store.get_circulating_supply() / 100_000_000}')
    # utxo_tips = store.get_utxo_tips()
    # print(f'get_utxo_tips() -> {utxo_tips}')

    # Get block that should exist
    block = store.get_block_header(utxo_tips[0], include_transactions=True)
    print(f'get_block_header() -> {block["header"].keys()}')

    # Get block that doesn't exist
    print(f'get_block_header() -> {store.get_block_header("04a368709289ddfc363c3b0bc4c6db97b1aa4376ebce93f0cd95e6f62e7a7492", include_transactions=True)}')