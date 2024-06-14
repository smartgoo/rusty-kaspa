from kaspapy import (
    Mnemonic, 
    XPrv, 
    DerivationPath,
    PublicKey
)


if __name__ == "__main__":
    # mnemonic = Mnemonic.random()
    mnemonic = Mnemonic('hunt bitter praise lift buyer topic crane leopard uniform network inquiry over grain pass match crush marine strike doll relax fortune trumpet sunny silk')
    print(mnemonic)
    seed = mnemonic.to_seed()
    print(seed)

    xprv = XPrv(seed)

    # derive full path upto second address of receive wallet
    pubkey1 = xprv.derive_path("m/44'/111111'/0'/0/1").to_xpub().to_public_key()
    print(pubkey1.to_address('kaspa'))

    # create receive wallet
    receive_wallet_xpub = xprv.derive_path("m/44'/111111'/0'/0").to_xpub()

    # derive receive wallet for second address
    pubkey2 = receive_wallet_xpub.derive_child(1, False).to_public_key()
    print(pubkey2.to_address('kaspa'))

    # create change wallet
    change_wallet_xpub = xprv.derive_path("m/44'/111111'/0'/1").to_xpub()
    
    # derive change wallet for first address
    pubkey3 = change_wallet_xpub.derive_child(0, False).to_public_key()
    print(pubkey3.to_address('kaspa'))

    # xprv with ktrv prefix
    ktrv = xprv.into_string('ktrv')
    print(ktrv)

    # create derivation path
    path = DerivationPath("m/1'")
    path.push(2, True)
    path.push(3, False)
    print(path)

    # derive by path string
    print(xprv.derive_path("m/1'/2'/3").into_string('xprv'))

    # derive by DerivationPath object
    # print(xprv.derive_path(path).into_string('xprv')) TODO not working

    # create XPrv from ktrvxxx string and derive it
    print(xprv.from_xprv(ktrv).derive_path("m/1'/2'/3").into_string("xprv"))

    # get xpub
    xpub = xprv.to_xpub()
    print(xpub.derive_path("m/1").into_string("xpub"))
    print(xpub.to_public_key().to_string())