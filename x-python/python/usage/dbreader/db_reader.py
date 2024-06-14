from pathlib import Path

from kaspapy import DBReader

if __name__ == "__main__":
    home_dir = Path.home()
    rk_app_dir = home_dir / '.rusty-kaspa'
    db = DBReader(app_dir=rk_app_dir)

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