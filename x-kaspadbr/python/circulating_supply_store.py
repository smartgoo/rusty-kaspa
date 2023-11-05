from kaspadbr import stores

if __name__ == "__main__":
    css = stores.CirculatingSupplyStore(app_dir='/opt2/kaspad-appdir/rust')

    print(css.get() / 100_000_000)
