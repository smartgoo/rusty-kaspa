import json

from kaspapy import GrpcClient

if __name__ == "__main__":
    c = GrpcClient(
        url='grpc://3.129.29.112:16110'
    )

    print(c.is_connected())
    r = c.get_server_info()
    print(json.loads(r))