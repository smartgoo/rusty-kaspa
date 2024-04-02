import asyncio

from kaspapy.grpc import AioClient

async def main():
    async with AioClient('localhost', 16110) as client:
        print(await client.get_info(), '\n')

        print(await client.get_blockdag_info(), '\n')
        
        # print(await client.get_block(
        #     hash='ccae338e58b51fd1a408ad6143064252215f9bb4d328f0a40ee2efd91876976a', 
        #     include_transactions=True
        # ))

if __name__ == '__main__':
    asyncio.run(main())