
# Compiling protos:
1. Copy protos:
- `cd x-python`
- `cp ../rpc/grpc/core/proto/messages.proto python/src/grpc/protos/`
- `cp ../rpc/grpc/core/proto/rpc.proto python/src/grpc/protos/`
- TODO p2p.proto

2. Compile protos: 
- `cd python/src/grpc`
- `python -m grpc_tools.protoc -I./protos --python_out=./protos --grpc_python_out=./protos ./protos/rpc.proto ./protos/messages.proto`