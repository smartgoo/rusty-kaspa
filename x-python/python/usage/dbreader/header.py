from pathlib import Path

from kaspapy import DBReader

if __name__ == "__main__":
    home_dir = Path.home()
    rk_app_dir = home_dir / '.rusty-kaspa'
    db = DBReader(app_dir=rk_app_dir)

    stores = db.stores

    utxo_tips = stores.utxo_index_tips.get()

    bh = stores.headers.get(block_hash=utxo_tips[0])

    print(bh)