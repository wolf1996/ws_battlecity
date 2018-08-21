import asyncio
import websockets
import time
import random
import string
import commands

#cmds = ['{"unit":2,"cmd":{"ChangeDirection":{"newdir":"Left"}}}',
#        '{"unit":2,"cmd":{"Move":{"direction":"Up"}}}',
#]

s = asyncio.Semaphore(2)

async def reciever(ws, s):
    while True:
        async with s:
            resp = await ws.recv()
            print("got message from server:")
            print(resp)

async def sender(ws, s):
    while True:
        async with s:
            cmd = commands.CommandEncoder().encode(commands.MessageContainer.get_random(4))
            print("sending command:")
            print(cmd)
            await ws.send(cmd)
            await asyncio.sleep(20)


async def client(loop):
    cookie = {'login': ''.join(random.choice(string.ascii_uppercase + string.digits) for _ in range(5))}
    async with websockets.connect("ws://127.0.0.1:3012", extra_headers=cookie) as ws:
        asyncio.ensure_future(reciever(ws, s))
        asyncio.ensure_future(sender(ws, s))
        await asyncio.sleep(10000000000)
    print("exit")

def main():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(client(loop))
    loop.run_forever()


if __name__ == "__main__":
    main()