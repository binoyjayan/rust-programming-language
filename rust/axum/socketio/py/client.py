#!/usr/bin/env python

import sys
import asyncio
import socketio

sio = socketio.AsyncClient()

disconnect_flag = False


@sio.event
async def auth(data):
    print(f"Auth: {data}")


@sio.event
async def response(data):
    print(f"Response: {data}")


@sio.event
async def disconnect():
    global disconnect_flag
    print("disconnected from server")
    disconnect_flag = True
    await sio.disconnect()


async def main():
    global disconnect_flag
    await sio.connect("http://localhost:8080")

    # First message
    await sio.emit("message", "Hello Server")

    while not disconnect_flag:
        try:
            await asyncio.sleep(1)
            await sio.emit("message", "Hello...")
        except asyncio.exceptions.CancelledError:
            disconnect_flag = True

    print("Exiting program")
    # await sio.disconnect()


if __name__ == "__main__":
    asyncio.run(main())


# pip install aiohttp python-socketio
