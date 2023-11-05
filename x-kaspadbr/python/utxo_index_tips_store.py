from kaspadbr import UtxoIndexTipsStore

if __name__ == "__main__":
    uits = UtxoIndexTipsStore(app_dir='/opt2/kaspad-appdir/rust')

    print(uits.get())