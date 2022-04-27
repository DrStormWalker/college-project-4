from dataclasses import dataclass, asdict
from multiprocessing.connection import Connection
from dacite import from_dict
from threading import Thread, ThreadError
from argparse import ArgumentParser
from multiprocessing import Pipe
from typing import Optional, ClassVar, TypeVar
import random
import string
import traceback
import socket
import logging
import json


T = TypeVar("T")


@dataclass
class Message:
    type: str
    data: dict | T


@dataclass
class ClientConnectionResponse:
    client_id: int
    TYPE: ClassVar[str] = "@response client/connection"


@dataclass
class CreateRoomRequest:
    max_clients: int
    send_port: int
    recv_port: int


@dataclass
class CreateRoomResponse:
    room_id: str
    TYPE: ClassVar[str] = "@response room/create"


@dataclass
class JoinRoomRequest:
    room_id: str
    send_port: int
    recv_port: int


@dataclass
class NetworkData:
    ip: str
    send_port: int
    recv_port: int


@dataclass
class ClientData:
    client_id: int
    network_data: NetworkData


@dataclass
class JoinRoomResponse:
    success: bool
    room_id: str
    msg: Optional[str]
    host_data: Optional[ClientData]
    TYPE: ClassVar[str] = "@response room/join"


@dataclass
class JoinRoomNotification(ClientData):
    TYPE: ClassVar[str] = "@notification room/join"


@dataclass
class Client:
    id: int
    tx: Connection
    ip: str
    port: int


@dataclass
class RoomClient:
    id: int
    send_port: int
    recv_port: int


ClientId = int


@dataclass
class RoomInstance:
    id: str
    max_clients: int
    host_id: int
    clients: dict[ClientId, RoomClient]


class Clients:
    def __init__(self):
        self.clients: dict[ClientId, Client] = {}

    def __contains__(self, item):
        return item in self.clients
    
    def __len__(self):
        return len(self.clients)

    def __getitem__(self, item: ClientId) -> Client:
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

    room_id = "fYLyWg"

    while room_id in rooms:
        room_id = generate_id()

    rooms[room_id] = RoomInstance(
        id=room_id,
        max_clients=request.max_clients,
        host_id=client.id,
        clients={
             client.id: RoomClient(
                id=client.id,
                send_port=request.send_port,
                recv_port=request.recv_port,
            ),
        },
    )

    print(f"Client {client.id} created room with id {room_id}")

    client.tx.send(asdict(Message(
        type=CreateRoomResponse.TYPE,
        data=CreateRoomResponse(
            room_id=room_id,
        ),
    )))


def join_room(request: JoinRoomRequest, client: Client):
    if request.room_id not in rooms:
        client.tx.send(asdict(Message(
            type=JoinRoomResponse.TYPE,
            data=JoinRoomResponse(
                success=False,
                room_id=request.room_id,
                msg="Room not found",
                host_data=None,
            ),
        )))
        return

    room = rooms[request.room_id]
    room_host = room.clients[room.host_id]
    host = clients[room.host_id]

    client.tx.send(asdict(Message(
        type=JoinRoomResponse.TYPE,
        data=JoinRoomResponse(
            success=True,
            room_id=request.room_id,
            msg=None,
            host_data=ClientData(
                client_id=client.id,
                network_data=NetworkData(
                    ip=host.ip,
                    send_port=room_host.send_port,
                    recv_port=room_host.recv_port,
                ),
            ),
        ),
    )))

    host.tx.send(asdict(Message(
        type=JoinRoomNotification.TYPE,
        data=JoinRoomNotification(
            client_id=client.id,
            network_data=NetworkData(
                ip=client.ip,
                send_port=request.send_port,
                recv_port=request.recv_port,
            ),
        ),
    )))

    rooms[request.room_id].clients[client.id] = RoomClient(
        id=client.id,
        send_port=request.send_port,
        recv_port=request.recv_port,
    )


def process_request(data: dict, client: Client):
    msg = from_dict(data_class=Message, data=data)
    client = ClientIndex(client.id)

    match msg.type:
        case "room/create":
            msg = from_dict(data_class=CreateRoomRequest, data=msg.data)
            create_room(msg, client())
        case "room/join":
            msg = from_dict(data_class=JoinRoomRequest, data=msg.data)
            join_room(msg, client())


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

    tx.send(asdict(Message(
        type=ClientConnectionResponse.TYPE,
        data=ClientConnectionResponse(
            client_id=client_id,
        ),
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
