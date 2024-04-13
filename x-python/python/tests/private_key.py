from secrets import token_bytes

from kaspapy import PrivateKey

if __name__ == "__main__":
    pk_hex = token_bytes(32).hex()
    print(pk_hex)
    private_key = PrivateKey(pk_hex)
    print(private_key.to_string())