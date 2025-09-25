import asyncio 

from kaspa import Resolver, RpcClient

async def main():
    client = RpcClient(resolver=Resolver())
    await client.connect()

    bt = await client.get_block_template(request={
        "payAddress": "kaspa:qpzpfwcsqsxhxwup26r55fd0ghqlhyugz8cp6y3wxuddc02vcxtjg75pspnwz",
        "extraData": list("test".encode("utf-8"))
    })

    bt['block']['header']['nonce'] = 1000023

    print(await client.submit_block(request={
        "block": bt['block'],
        "allowNonDaaBlocks": True
    }))

if __name__ == "__main__":
    asyncio.run(main())