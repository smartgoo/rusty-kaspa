from kaspapy import Mnemonic, XPrv, DerivationPath


if __name__ == "__main__":
    mnemonic = Mnemonic.random()
    print(f'mnemonic: {mnemonic}')

    seed = mnemonic.to_seed()
    print(f'seed: {seed}')

    xprv = XPrv(seed)
    print(f'xPrv {xprv.into_string("xprv")}')

    x = xprv.derive_path("m/1'/2'/3").into_string("xprv")
    print(f'xPrv {x}')

    path = DerivationPath("m/1'")
    path.push(2, True)
    path.push(3, False)
    print(f'path {path.to_string()}')

    print(f'xPrv {xprv.derive_path(path.to_string()).into_string("xprv")}')

    xpub = xprv.public_key()
    print(f'xPub {xpub.derive_path("m/1").into_string("xpub")}')