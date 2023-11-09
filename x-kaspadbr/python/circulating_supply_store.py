from kaspadbr import Reader

if __name__ == "__main__":
    db = Reader(app_dir='/opt2/kaspad-appdir/rust')

    print(db.stores.circulating_supply_store.get() / 100_000_000)