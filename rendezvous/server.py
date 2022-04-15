from dataclasses import dataclass, asdict
from multiprocessing.connection import Connection
from dacite import from_dict
from threading import Thread, ThreadError
from argparse import ArgumentParser
from multiprocessing import Pipe
from typing import Optional
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
class ClientConnectionResponse:
    client_id: int


@dataclass
class CreateRoomRequest:
    max_clients: int
    known_port: int


@dataclass
class CreateRoomResponse:
    room_id: str


@dataclass
class JoinRoomRequest:
    room_id: str


@dataclass
class NetworkData:
    peer_ip: str
    peer_port: int
    known_port: int


@dataclass
class JoinRoomResponse:
    success: bool
    msg: Optional[str]
    data: Optional[NetworkData]


@dataclass
class Client:
    id: int
    tx: Connection
    ip: str
    port: int


@dataclass
class RoomInstance:
    id: str
    max_clients: int
    connection_host: str
    connection_port: int
    known_port: int
    host_id: int
    clients: list[int]


ClientId = int


class Clients:
    def __init__(self):
        self.clients: dict[ClientId, Client] = {}

    def __contains__(self, item):
        return item in self.clients

    def __getitem__(self, item: ClientId):
        return self.clients[item]

    def __setitem__(self, key: ClientId, value: Client):
        self.clients[key] = value


clients: Clients = Clients()


class ClientIndex:
    def __init__(self, client_id: ClientId):
        self.client_id = client_id

    def __call__(self, *args, **kwargs):
        return clients[self.client_id]


rooms: dict[str, RoomInstance] = {}

ROOM_ID_CHARS = string.ascii_letters


def create_room(request: CreateRoomRequest, client: Client):
    def generate_id():
        return ''.join(random.choice(ROOM_ID_CHARS) for _ in range(6))

    room_id = generate_id()

    while room_id in rooms:
        room_id = generate_id()

    rooms[room_id] = RoomInstance(
        id=room_id,
        max_clients=request.max_clients,
        connection_host=client.ip,
        connection_port=client.port,
        known_port=request.known_port,
        host_id=client.id,
        clients=[
            client.id,
        ],
    )

    print(f"Client {client.id} created room with id {room_id}")

    client.tx.send(asdict(CreateRoomResponse(
        room_id=room_id,
    )))


def join_room(request: JoinRoomRequest, client: ClientIndex):
    if not request.room_id in rooms:
        client().tx.send(asdict(JoinRoomResponse(
            success=False,
            msg="Room not found",
            data=None,
        )))
        return

    room = rooms[request.room_id]

    client().tx.send(asdict(JoinRoomResponse(
        success=True,
        msg=None,
        data=NetworkData(
            peer_ip=room.connection_host,
            peer_port=room.connection_port,
            known_port=room.known_port,
        ),
    )))

    ClientIndex(room.host_id)().tx.send(asdict(JoinRoomResponse(
        success=True,
        msg=None,
        data=NetworkData(
            peer_ip=client().ip,
            peer_port=client().port,
            known_port=room.known_port,
        ),
    )))


def process_request(data: dict, client: Client):
    msg = from_dict(data_class=ClientMessage, data=data)
    client = ClientIndex(client.id)

    match msg.type:
        case "room/create":
            msg = from_dict(data_class=CreateRoomRequest, data=msg.data)
            create_room(msg, client())
        case "room/join":
            msg = from_dict(data_class=JoinRoomRequest, data=msg.data)
            join_room(msg, client)


def client_outbound(connection: socket.socket, rx: Connection):
    while True:
        data = rx.recv()

        if type(data) is dict:
            data = json.dumps(data).encode("utf-8")
        elif type(data) is str:
            data = data.encode("utf-8")

        bytes_sent = connection.send(data)


def client_inbound(connection: socket.socket, client: Client, max_buffer_size=4096):
    while True:
        data = connection.recv(max_buffer_size)
        data = json.loads(data)

        process_request(data, client)


def client_handler(connection: socket.socket, ip: str, port: int, max_buffer_size=4096):
    def generate_client_id():
        return int.from_bytes(random.randbytes(4), "big")

    client_id = generate_client_id()

    while client_id in clients:
        client_id = generate_client_id()

    rx, tx = Pipe()

    client = Client(
        id=client_id,
        tx=tx,
        ip=ip,
        port=port,
    )
    clients[client_id] = client

    Thread(
        target=client_outbound,
        args=(connection, rx),
    ).start()

    tx.send(asdict(ClientConnectionResponse(
        client_id=client_id,
    )))

    client_inbound(connection, client, max_buffer_size)


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
        default=50000,
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
