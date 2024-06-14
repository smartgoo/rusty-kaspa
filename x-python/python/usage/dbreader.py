from pathlib import Path

from kaspadbr import DBReader

if __name__ == "__main__":
    home_dir = Path.home()
    rk_app_dir = home_dir / '.rusty-kaspa'

    db = DBReader(app_dir=rk_app_dir)

    stores = db.stores

    # Directories
    print('\n')
    print('---------- DIRS')
    print(f'Home Dir: {db.home_dir}')
    print(f'App Dir: {db.app_dir}')
    print(f'Network Dir: {db.network_dir}')
    print(f'DB Dir: {db.db_dir}')
    print(f'UTXO DB Dir: {db.utxo_index_db_dir}')
    print(f'Meta DB Dir: {db.meta_db_dir}')
    print(f'Consensus DB Dir: {db.consensus_db_dir}')

    print('\n---------- STORES')

    r = stores.metadata.current_consensus_key()
    print(f'metadata.current_consensus_key() -> {r}\n')

    r = stores.circulating_supply.get()
    print(f'circulating_supply.get() -> {r / 100_000_000}\n')

    utxo_tips = stores.utxo_index_tips.get()
    print(f'utxo_index_tips.get() -> {utxo_tips}\n')

    r = stores.headers.get(block_hash=utxo_tips[0])
    del r['parents_by_level'] # to save space on print
    print(f'headers.get(block_hash="{utxo_tips[0]}") -> {r}\n')

    # Export entire UTXO index to CSV file
    c = stores.utxo_index.export(
        filepath=f"/{home_dir}/utxo_set_of_{utxo_tips[0]}.csv", 
        address=True,       # Optional, default True
        daa_score=True,     # Optional, default True
        amount=True,        # Optional, default True
        is_coinbase=True,   # Optional, default True
        outpoint=False,     # Optional, default False
        chunk_size=100000,  # Optional, default 100000
        verbose=True        # Optional, default False
    )
    print(f'utxo_index.export() -> exported {c} UTXO records to CSV.')

    # Export address balances to CSV file
    c = stores.utxo_index.export_addresses(
        filepath=f"/{home_dir}/unique_addresses_of_{utxo_tips[0]}.csv", 
        verbose=True
    )
    print(f'utxo_index.export_addresses() -> exported {c} addresses records to CSV.')


