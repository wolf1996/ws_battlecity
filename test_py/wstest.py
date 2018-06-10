import asyncio
import websockets
import time
import random
import string
import commands

#cmds = ['{"unit":2,"cmd":{"ChangeDirection":{"newdir":"Left"}}}',
#        '{"unit":2,"cmd":{"Move":{"direction":"Up"}}}',
#]
async def client():
    cookie = {'login': ''.join(random.choice(string.ascii_uppercase + string.digits) for _ in range(5))}
    async with websockets.connect("ws://127.0.0.1:3012", extra_headers=cookie) as ws:
        for j in range(1,2):
            resp = await ws.recv()
            print(resp)
        i = 0
        for i in range(1,10):
            cmd = commands.CommandEncoder().encode(commands.MessageContainer.get_random(4))
            print(cmd)
            await ws.send(cmd)
        print("send finished \n starting to recieve")
        j = 0
        for j in range(1,100):
            resp = await ws.recv()
            print(resp)
    print("exit")

def main():
    asyncio.get_event_loop().run_until_complete(client())

if __name__ == "__main__":
    main()