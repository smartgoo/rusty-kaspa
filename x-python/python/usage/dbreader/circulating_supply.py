from pathlib import Path

from kaspapy import DBReader

if __name__ == "__main__":
    home_dir = Path.home()
    rk_app_dir = home_dir / '.rusty-kaspa'
    db = DBReader(app_dir=rk_app_dir)

    stores = db.stores

    cs = stores.circulating_supply.get()
    print(cs / 100_000_000)
