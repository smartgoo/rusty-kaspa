from kaspadbr import Reader

if __name__ == "__main__":
    store = Reader(app_dir='/opt2/kaspad-appdir/rust')

    utxo_tips = store.get_utxo_tips()

    for tip in utxo_tips:
        h = store.get_block_header(tip)
        print(h['hash'], h['timestamp'], h['daa_score'])
    
    print(store.export_utxo_set(filepath="/opt2/work/dev/test.csv", verbose=True))