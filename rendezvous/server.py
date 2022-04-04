from dataclasses import dataclass, asdict
from multiprocessing.connection import Connection
from dacite import from_dict
from threading import Thread, ThreadError
from argparse import ArgumentParser
from multiprocessing import Pipe
import random
import string
import traceback
import socket
import logging
import json

@dataclass
class ClientMessage:
    type: str
    data: dict

@dataclass
class RoomRequest:
    max_clients: int

@dataclass
class Client:
    id: int
    tx: Connection

@dataclass
class RoomInstance:
    id: str
    max_clients: int
    clients: list[Client]

rooms: dict[str, RoomInstance] = {}
clients: dict[int, Client] = {}

ROOM_ID_CHARS = string.ascii_letters

def create_room(request: RoomRequest, client: Client):
    def generate_id():
        return ''.join(random.choice(ROOM_ID_CHARS) for _ in range(6))

    room_id = generate_id()

    while room_id in rooms:
        room_id = generate_id()

    rooms[room_id] = RoomInstance(
        id=room_id,
        max_clients=request.max_clients,
        clients=[

        ],
    )

def process_request(data: dict, client: Client):
    msg = from_dict(data_class=ClientMessage, data=data)

    match msg.type:
        case "room_request":
            msg = from_dict(data_class=RoomRequest, data=msg.data)
            create_room(msg, client)

def client_handler(connection: socket.socket, ip: str, port: int, max_buffer_size = 4096):
    def generate_client_id():
        return int(random.randbytes(4))

    client_id = generate_client_id()

    while client_id in clients:
        client_id = generate_client_id()

    rx, tx = Pipe()

    client = Client(
        id=client_id,
        tx=tx,
    )
    client[client_id] = client

    while True:
        data = connection.recv(max_buffer_size)

        data = data.decode("utf-8").rstrip()
        data = json.loads(data)
        process_request(data, client)

def main():
    parser = ArgumentParser(description="Team Platformer rendezvous server")
    parser.add_argument(
        "-s", "--source", "--host",
        help="Specifies the source address for the socket",
        type=str,
        default="127.0.0.1",
        dest="host",
    )
    parser.add_argument(
        "-p", "--port",
        help="Specifies the port for the server to listen on",
        type=int,
        default=50001,
        dest="port",
    )

    args = parser.parse_args()
    host: str = args.host
    port: int = args.port

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as soc:
        soc.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        logging.info("Created socket")

        soc.bind((host, port))

        soc.listen()

        while True:
            connection, address = soc.accept()
            client_ip, client_port = str(address[0]), int(address[1])
            logging.info(f"Received connection from {client_ip}:{client_port}")

            try:
                Thread(
                    target=client_handler,
                    args=(connection, client_ip, client_port),
                ).start()
            except ThreadError:
                logging.error("Failed to start thread")
                traceback.print_exc()

if __name__ == "__main__":
    main()