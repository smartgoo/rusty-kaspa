from kaspapy import DBReader

if __name__ == "__main__":
    db = DBReader(app_dir='/data/rusty-kaspa')

    stores = db.stores

    cs = stores.circulating_supply.get()
    print(cs / 100_000_000)
