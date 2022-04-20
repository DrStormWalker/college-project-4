use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs, UdpSocket};
use tokio::sync::{broadcast, mpsc, Mutex};

#[derive(Debug)]
pub enum IoOrSerdeError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
}
impl std::error::Error for IoOrSerdeError {}
impl Display for IoOrSerdeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{}", e),
            Self::SerdeError(e) => write!(f, "{}", e),
        }
    }
}
impl From<io::Error> for IoOrSerdeError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}
impl From<serde_json::Error> for IoOrSerdeError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(e)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub data: Value,
}
impl Message {
    pub fn new<'a>(msg_type: String, data: impl Serialize) -> Self {
        Self {
            msg_type,
            data: serde_json::to_value(data).unwrap(),
        }
    }
}
impl TryInto<Value> for Message {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<Value, Self::Error> {
        serde_json::to_value(self)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClientJoinedResponse {
    pub client_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub max_clients: usize,
    pub send_port: u16,
    pub recv_port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRoomResponse {
    pub room_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct JoinRoomRequest {
    pub room_id: String,
    pub send_port: u16,
    pub recv_port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkData {
    ip: String,
    send_port: u16,
    recv_port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct ClientData {
    client_id: u32,
    network_data: NetworkData,
}

#[derive(Serialize, Deserialize)]
pub struct JoinRoomResponse {
    pub success: bool,
    pub room_id: String,
    pub msg: Option<String>,
    pub host_data: Option<ClientData>,
}

type JoinRoomNotification = ClientData;

fn print_err<O, E>(e: E) -> O
where
    O: Default,
    E: Display,
{
    println!("{}", e);

    O::default()
}

struct RendezvousConnector {
    pub tx: mpsc::Sender<Message>,
    pub client_id: u32,
}

#[derive(Debug)]
pub struct PeerConnection {
    pub tx: mpsc::Sender<Message>,
    pub client_id: u32,
    pub peer_addr: SocketAddr,
}

#[derive(Debug)]
pub enum RoomConnectionType {
    Host(Vec<PeerConnection>),
    Client(PeerConnection),
}

#[derive(Debug)]
pub struct RoomConnection {
    pub room_id: String,
    pub room_host: u32,
    pub connection_type: RoomConnectionType,
}

pub struct CommunicationSockets {
    tx: Arc<UdpSocket>,
    rx: Arc<UdpSocket>,
}

pub struct TransmissionNetworkPortal {
    rendezvous_connection: Option<RendezvousConnector>,
    pub room_connection: Option<RoomConnection>,
    sockets: Option<CommunicationSockets>,
}
impl TransmissionNetworkPortal {
    pub fn new() -> Self {
        Self {
            rendezvous_connection: None,
            room_connection: None,
            sockets: None,
        }
    }

    async fn handle_rendezvous_message(
        this: Arc<Mutex<Self>>,
        msg: Message,
        broadcast_tx: broadcast::Sender<Message>,
    ) -> Result<(), IoOrSerdeError> {
        println!("Received rendezvous message: {:?}", msg);
        let msg_type = msg.msg_type.as_str();

        match msg_type {
            "@response room/create" => {
                let msg = serde_json::from_value::<CreateRoomResponse>(msg.data)?;

                let mut this = this.lock().await;

                this.room_connection = Some(RoomConnection {
                    room_id: msg.room_id,
                    room_host: this.rendezvous_connection.as_mut().unwrap().client_id,
                    connection_type: RoomConnectionType::Host(vec![]),
                });

                println!("Room connection established");
            }
            "@response room/join" => {
                let msg = serde_json::from_value::<JoinRoomResponse>(msg.data)?;

                if !msg.success {
                    panic!("Failed to join room");
                }

                let host_data = msg.host_data.unwrap();

                let ip: IpAddr = host_data.network_data.ip.parse().unwrap();
                let send_port = host_data.network_data.send_port;
                let recv_port = host_data.network_data.recv_port;

                let peer_send_addr = SocketAddr::new(ip, send_port);
                let peer_recv_addr = SocketAddr::new(ip, recv_port);

                let mut this = this.lock().await;

                let sockets = this.sockets.as_ref().unwrap();
                let send_socket = sockets.tx.clone();
                let recv_socket = sockets.rx.clone();

                let tx = {
                    let msg = Message::new("connection/hole-punch".to_string(), ());
                    let msg = serde_json::to_string(&msg)?;

                    recv_socket.send_to(msg.as_bytes(), peer_send_addr).await?;

                    tokio::spawn(async move {
                        loop {
                            let msg = Message::new("connection/keep-alive".to_string(), ());
                            let msg = serde_json::to_string(&msg).unwrap();
                            recv_socket.send_to(msg.as_bytes(), peer_send_addr).await;

                            tokio::time::sleep(Duration::from_secs(5)).await;
                        }
                    });

                    let socket = send_socket.clone();

                    let (tx, mut rx) = mpsc::channel::<Message>(100);

                    tokio::spawn(async move {
                        while let Some(val) = rx.recv().await {
                            let msg = serde_json::to_string(&val).unwrap();

                            socket
                                .send_to(msg.as_bytes(), peer_recv_addr)
                                .await
                                .unwrap_or_else(print_err);
                        }
                    });

                    {
                        let mut rx = broadcast_tx.subscribe();
                        let tx = tx.clone();
                        tokio::spawn(async move {
                            while let Ok(msg) = rx.recv().await {
                                tx.send(msg).await;
                            }
                        });
                    }

                    tx
                };

                {
                    let recv = recv_socket.clone();

                    tokio::spawn(async move {
                        let mut buf = [0u8; 4096];
                        while let Ok((size, addr)) = recv.recv_from(&mut buf).await {
                            let msg = String::from_utf8_lossy(&buf[..size]);

                            buf = [0u8; 4096];
                        }
                    });
                }

                this.room_connection = Some(RoomConnection {
                    room_id: msg.room_id,
                    room_host: host_data.client_id,
                    connection_type: RoomConnectionType::Client(PeerConnection {
                        tx,
                        client_id: host_data.client_id,
                        peer_addr: SocketAddr::new(
                            host_data.network_data.ip.parse().unwrap(),
                            host_data.network_data.send_port,
                        ),
                    }),
                });
            }
            "@notification room/join" => {
                let msg = serde_json::from_value::<JoinRoomNotification>(msg.data)?;

                let ip: IpAddr = msg.network_data.ip.parse().unwrap();
                let send_port = msg.network_data.send_port;
                let recv_port = msg.network_data.recv_port;

                let peer_send_addr = SocketAddr::new(ip, send_port);
                let peer_recv_addr = SocketAddr::new(ip, recv_port);

                let mut this = this.lock().await;

                if let RoomConnectionType::Host(_) =
                    &this.room_connection.as_ref().unwrap().connection_type
                {
                } else {
                    panic!("Received a room/join notification while not a host")
                }

                let sockets = this.sockets.as_ref().unwrap();
                let send_socket = sockets.tx.clone();
                let recv_socket = sockets.rx.clone();

                let tx = {
                    let client_id = msg.client_id;

                    let msg = Message::new("connection/hole-punch".to_string(), ());
                    let msg = serde_json::to_string(&msg)?;

                    recv_socket.send_to(msg.as_bytes(), peer_send_addr).await?;

                    tokio::spawn(async move {
                        loop {
                            let msg = Message::new("connection/keep-alive".to_string(), ());
                            let msg = serde_json::to_string(&msg).unwrap();
                            recv_socket.send_to(msg.as_bytes(), peer_send_addr).await;

                            tokio::time::sleep(Duration::from_secs(5)).await;
                        }
                    });

                    let socket = send_socket.clone();

                    let (tx, mut rx) = mpsc::channel::<Message>(100);

                    tokio::spawn(async move {
                        while let Some(val) = rx.recv().await {
                            let msg = serde_json::to_string(&val).unwrap();

                            socket
                                .send_to(msg.as_bytes(), peer_recv_addr)
                                .await
                                .unwrap_or_else(print_err);
                        }
                    });

                    {
                        let mut rx = broadcast_tx.subscribe();
                        let tx = tx.clone();
                        tokio::spawn(async move {
                            while let Ok(msg) = rx.recv().await {
                                tx.send(msg).await;
                            }
                        });
                    }

                    tx
                };

                match &mut this.room_connection.as_mut().unwrap().connection_type {
                    RoomConnectionType::Host(client_connections) => {
                        println!("Client {} joined the room", msg.client_id);
                        client_connections.push(PeerConnection {
                            tx,
                            client_id: msg.client_id,
                            peer_addr: SocketAddr::new(ip, msg.network_data.send_port),
                        });
                    }
                    _ => panic!("Received a room/join notification while not a host"),
                }
            }
            _ => println!("Received unknown message type {}", msg_type),
        }

        Ok(())
    }

    pub async fn rendezvous_init<A>(
        mut self,
        addr: A,
        source: IpAddr,
        send_port: u16,
        recv_port: u16,
    ) -> Result<
        (
            Arc<Mutex<Self>>,
            (broadcast::Sender<Message>, mpsc::Receiver<Message>),
        ),
        IoOrSerdeError,
    >
    where
        A: ToSocketAddrs,
    {
        let (tx, mut rx) = mpsc::channel::<Message>(100);

        let stream = TcpStream::connect(addr).await?;
        let (mut stream_rx, mut stream_tx) = stream.into_split();

        let mut buf = [0u8; 4096];
        let size = stream_rx.read(&mut buf).await?;

        let response = serde_json::from_slice::<Message>(&buf[..size])?;
        let response = serde_json::from_value::<ClientJoinedResponse>(response.data)?;

        self.rendezvous_connection = Some(RendezvousConnector {
            tx: tx.clone(),
            client_id: response.client_id,
        });

        println!(
            "Successfully registered to rendezvous server as {}",
            response.client_id
        );

        let send_socket = UdpSocket::bind(SocketAddr::new(source, send_port)).await?;
        let send_socket = Arc::new(send_socket);

        let recv_socket = UdpSocket::bind(SocketAddr::new(source, recv_port)).await?;
        let recv_socket = Arc::new(recv_socket);

        self.sockets = Some(CommunicationSockets {
            tx: send_socket,
            rx: recv_socket.clone(),
        });

        let this = Arc::new(Mutex::new(self));

        tokio::spawn(async move {
            while let Some(val) = rx.recv().await {
                let value = serde_json::to_string(&val).unwrap();

                println!("{}", value);

                stream_tx
                    .write_all(value.as_bytes())
                    .await
                    .unwrap_or_else(print_err);
            }
        });

        let (tx, _) = broadcast::channel::<Message>(5);

        {
            let tx = tx.clone();
            let this = this.clone();
            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                while let Ok(size) = stream_rx.read(&mut buf).await {
                    let msg = match serde_json::from_slice::<Message>(&buf[..size]) {
                        Ok(msg) => msg,
                        Err(e) => {
                            println!(
                                "Failed to parse json: {:?}, {}",
                                String::from_utf8_lossy(buf.as_slice()),
                                e
                            );
                            continue;
                        }
                    };

                    Self::handle_rendezvous_message(this.clone(), msg, tx.clone())
                        .await
                        .unwrap_or_else(print_err);

                    buf = [0u8; 4096];
                }
            });
        }

        let rx = {
            let (tx, rx) = mpsc::channel(100);
            let socket = recv_socket;

            tokio::spawn(async move {
                let mut buf = [0; 4096];
                while let Ok(size) = socket.recv(&mut buf).await {
                    let msg = serde_json::from_slice::<Message>(&buf[0..size]).unwrap();

                    tx.send(msg).await;

                    buf = [0; 4096];
                }
            });

            rx
        };

        Ok((this, (tx, rx)))
    }

    pub async fn create_room(&mut self) -> Result<(), IoOrSerdeError> {
        let tx = &self.rendezvous_connection.as_ref().unwrap().tx;

        let sockets = &self.sockets.as_ref().unwrap();
        let send_port = sockets.tx.local_addr().unwrap().port();
        let recv_port = sockets.rx.local_addr().unwrap().port();

        let data = serde_json::to_value(CreateRoomRequest {
            max_clients: 2,
            send_port,
            recv_port,
        })?;

        tx.send(
            Message::new("room/create".to_string(), data)
                .try_into()
                .unwrap(),
        )
        .await
        .unwrap_or_else(print_err);

        Ok(())
    }

    pub async fn join_room(&mut self, room_id: String) -> Result<(), IoOrSerdeError> {
        let tx = &self.rendezvous_connection.as_ref().unwrap().tx;

        let sockets = &self.sockets.as_ref().unwrap();
        let send_port = sockets.tx.local_addr().unwrap().port();
        let recv_port = sockets.rx.local_addr().unwrap().port();

        let data = serde_json::to_value(JoinRoomRequest {
            room_id,
            send_port,
            recv_port,
        })?;

        tx.send(
            Message::new("room/join".to_string(), data)
                .try_into()
                .unwrap(),
        )
        .await
        .unwrap_or_else(print_err);

        Ok(())
    }
}
