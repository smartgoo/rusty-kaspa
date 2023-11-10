from kaspadbr import DBReader

if __name__ == "__main__":
    db = DBReader(app_dir='/opt2/kaspad-appdir/rust')

    cs_store = db.stores.circulating_supply
    cs = cs_store.get()
    print(cs / 100_000_000)
