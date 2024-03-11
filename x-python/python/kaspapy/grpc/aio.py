import asyncio
import logging
import functools

import grpc
from google.protobuf import json_format

from .protos import messages_pb2_grpc
from .protos import rpc_pb2
from .protos.messages_pb2 import KaspadRequest

MAX_MESSAGE_LENGTH = 1024 * 1024 * 1024 # 1 GB

def with_rpc_params(func):
    @functools.wraps(func)
    async def wrapper(self, *args, retries: int=0, timeout: int=5, **kwargs):
        return await func(self, *args, retries=retries, timeout=timeout, **kwargs)
    return wrapper

class AioClient:
    def __init__(self, host: str, port: int):
        self.host = host
        self.port = port

        self.channel = grpc.aio.insecure_channel(
            f'{self.host}:{self.port}',
            compression=grpc.Compression.Gzip,
            options=[
                ('grpc.max_send_message_length', MAX_MESSAGE_LENGTH),
                ('grpc.max_receive_message_length', MAX_MESSAGE_LENGTH),
            ]
        )
        self.stub = messages_pb2_grpc.RPCStub(self.channel)

    async def __aenter__(self):
        """ Async context manager entry """
        return self

    async def __aexit__(self, exc_type, exc, tb):
        """ Async context manager exit """
        await self.channel.close()

    async def close_channel(self):
        """ Closes persistent channel """
        await self.channel.close()
        self.channel = None 

    def _request_iterator(self, request):
        yield request

    async def _send_request(self, request, retries: int = 0, timeout: int = 5):
        for i in range(retries + 1):
            try:
                responses = []
                async for response in self.stub.MessageStream(self._request_iterator(request), timeout=timeout):
                    responses.append(response)
                return responses[0] if len(responses) == 1 else responses
            except grpc.RpcError as e:
                # TODO catch various errors and handle accordingly
                if i >= retries:
                    logging.error(f"RPC failed after {retries} retries: {e}")
                    return None
                await asyncio.sleep(2 ** i)

    async def _handle(self, cmd, payload=None, *args, **kwargs):
        wrapped_request = KaspadRequest()
        request = getattr(wrapped_request, cmd)

        if payload:
            if isinstance(payload, dict):
                json_format.ParseDict(payload, request)
            if isinstance(payload, str):
                json_format.Parse(payload, request)

        request.SetInParent()
        response = await self._send_request(wrapped_request, **kwargs)
        return json_format.MessageToDict(response) if response else None

    # @with_rpc_params
    # async def get_current_network():
    #     """ GetCurrentNetworkRequestMessage """
    #     pass
    
    # @with_rpc_params
    # async def get_block_template():
    #     """ GetBlockTemplateRequestMessage """
    #     pass

    # @with_rpc_params
    # async def get_peer_addresses():
    #     """ GetPeerAddressesRequestMessage """
    #     pass
    
    # @with_rpc_params
    # async def get_sink():
    #     """ GetSinkRequestMessage """
    #     pass
    
    # @with_rpc_params
    # async def get_mempool_entry():
    #     """ GetMempoolEntryRequestMessage """
    #     pass
    
    # @with_rpc_params
    # async def get_connected_peer_info():
    #     """ GetConnectedPeerInfoRequestMessage """
    #     pass
    
    @with_rpc_params
    async def get_block(self, hash: str, include_transactions: bool, **kwargs):
        """ GetBlockRequestMessage """
        payload = {
            'hash': hash,
            'includeTransactions': include_transactions
        }

        return await self._handle('getBlockRequest', payload, **kwargs)
        
    # @with_rpc_params
    # async def get_subnetwork():
    #     """ GetSubnetworkRequestMessage """
    #     pass
    
    # @with_rpc_params
    # async def get_virtual_chain_from_block():
    #     """ GetVirtualChainFromBlockRequestMessage """
    #     pass
    
    # @with_rpc_params
    # async def get_blocks():
    #     """ GetBlocksRequestMessage """
    #     pass

    # @with_rpc_params
    # async def get_block_count():
    #     """ GetBlockCountRequestMessage """
    #     pass

    @with_rpc_params
    async def get_blockdag_info(self, **kwargs):
        """ GetBlockDagInfoRequestMessage """
        return await self._handle('getBlockDagInfoRequest', **kwargs)
        # return GetBlockDagInfoResponse(**r['getBlockDagInfoResponse']) if r else None
    
    # @with_rpc_params
    # async def get_headers():
    #     """ GetHeadersRequestMessage """
    #     pass

    # @with_rpc_params
    # async def get_utxos_by_address():
    #     """ GetUtxosByAddressesRequestMessage """
    #     pass

    # @with_rpc_params
    # async def get_balance_by_address():
    #     """ GetBalanceByAddressRequestMessage """
    #     pass

    # @with_rpc_params
    # async def get_balance_by_addresses():
    #     """ GetBalancesByAddressesRequestMessage """
    #     pass

    # @with_rpc_params
    # async def get_sink_blue_score():
    #     """ GetSinkBlueScoreRequestMessage """
    #     pass

    @with_rpc_params
    async def get_info(self, **kwargs):
        """ GetInfoRequestMessage """
        return await self._handle('getInfoRequest', **kwargs)
        # return GetInfoResponse(**r['getInfoResponse']) if r else None

    # @with_rpc_params
    # async def get_mempool_entries_by_addresses():
    #     """ GetMempoolEntriesByAddressesRequestMessage """
    #     pass

    # @with_rpc_params
    # async def get_coin_supply():
    #     """ GetCoinSupplyRequestMessage """
    #     pass

    # @with_rpc_params
    # async def get_metrics():
    #     """ GetMetricsRequestMessage """
    #     pass

    # @with_rpc_params
    # async def get_server_info():
    #     """ GetServerInfoRequestMessage """
    #     pass
    
    # @with_rpc_params
    # async def get_sync_status():
    #     """ GetSyncStatusRequestMessage """
    #     pass