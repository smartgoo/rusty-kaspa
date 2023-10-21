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
    print(f'get_circulating_supply() -> {store.get_circulating_supply() / 100_000_000}')
    print(f'get_utxo_tips() -> {store.get_utxo_tips()}')
    print(f'get_block_header() -> {store.get_block_header("5e61c3af59255cdd6362b72b8300f29a50580f42f40641996213a4f33af67c30")}')