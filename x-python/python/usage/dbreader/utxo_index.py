from pathlib import Path

from kaspapy import DBReader

if __name__ == "__main__":
    home_dir = Path.home()
    rk_app_dir = home_dir / '.rusty-kaspa'
    db = DBReader(app_dir=rk_app_dir)

    stores = db.stores
    utxo_tips = stores.utxo_index_tips.get()

    for tip_hash in utxo_tips:
        h = stores.headers.get(tip_hash)
        print(h['hash'], h['timestamp'], h['daa_score'])

    # Export UTXO set to CSV file
    c = stores.utxo_index.export(filepath=f"/{home_dir}/utxo_set_of_{utxo_tips[0]}.csv", outpoint=False, verbose=True)
    print(f'Exported {c} UTXO records to CSV.')

    # Export addresses to CSV file
    c = stores.utxo_index.export_addresses(filepath=f"/{home_dir}/unique_addresses_of_{utxo_tips[0]}.csv", verbose=True)
    print(c)
