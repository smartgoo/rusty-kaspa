from kaspapy import Mnemonic

if __name__ == "__main__":    
    m1 = Mnemonic.random()
    print(m1.phrase)

    m2 = Mnemonic(m1.phrase)
    print(m2.phrase)

    seed1 = m1.to_seed("test_password!")
    print(seed1)

    seed2 = m2.to_seed("test_password!")
    print(seed2)

    seed3 = m1.to_seed()
    print(seed3)