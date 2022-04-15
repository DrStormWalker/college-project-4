use std::fmt::{Display, Formatter, write};
use std::io::Error;
use std::net::SocketAddr;
use serde_json::Value;
use serde_derive::{Serialize, Deserialize};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use specs::System;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io;
use tokio::sync::mpsc::Sender;

macro_rules! json_map{
    ( $($key:tt => $val:expr),* $(,)? ) => {{
        #[allow(unused_mut)]
        let mut map = serde_json::Map::with_capacity(json_map!(@count $($key),* ));
        $(
            #[allow(unused_parens)]
            let _ = map.insert($key.into(), serde_json::Value::from($val));
        )*
        map
    }};
    (@replace $_t:tt $e:expr ) => { $e };
    (@count $($t:tt)*) => { <[()]>::len(&[$( json_map!(@replace $t ()) ),*]) }
}

#[derive(Debug)]
pub enum IoOrSerdeError {
    IoError(io::Error),
    SerdeError(serde_json::Error)
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

#[derive(Serialize, Deserialize)]
pub struct ClientJoinedResponse {
    pub client_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub max_clients: usize,
    pub known_port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct JoinRoomRequest {
    pub room_id: String,
}

struct RendezvousConnector {
    pub tx: Sender<Value>,
    pub client_id: u32,
}

fn print_err<E>(e: E)
where
    E: Display
{
    println!("{}", e)
}

pub struct TransmissionNetworkPortal {
    rendezvous_connection: Option<RendezvousConnector>,
}
impl TransmissionNetworkPortal {
    pub fn new() -> Self {
        Self {
            rendezvous_connection: None,
        }
    }

    pub async fn rendezvous_init(&mut self, addr: SocketAddr) -> Result<(), IoOrSerdeError> {

        let (tx, mut rx) = mpsc::channel::<Value>(100);

        let stream = TcpStream::connect(addr).await?;
        let (mut stream_rx, mut stream_tx) = stream.into_split();

        let mut buf = Vec::with_capacity(4096);
        stream_rx.read_buf(&mut buf).await?;
        let response = String::from_utf8(buf.clone()).unwrap();

        let response = serde_json::from_str::<ClientJoinedResponse>(response.as_str())?;

        self.rendezvous_connection = Some(RendezvousConnector {
            tx: tx.clone(),
            client_id: response.client_id,
        });

        println!("Successfully registered to rendezvous server as {}", response.client_id);

        tokio::spawn(async move {
            while let Some(val) = rx.recv().await {
                let value = serde_json::to_string(&val).unwrap();

                println!("{}", value);

                stream_tx.write_all(value.as_bytes())
                    .await
                    .unwrap_or_else(print_err);
            }
        });

        tokio::spawn(async move {
            while let Ok(size) = stream_rx.read_buf(&mut buf).await {
                println!("{}", String::from_utf8(buf.clone()).unwrap());
            }
        });

        Ok(())
    }

    pub async fn create_room(&mut self, known_port: u16) -> Result<(), IoOrSerdeError> {
        let tx = match &self.rendezvous_connection {
            Some(conn) => conn.tx.clone(),
            None => panic!(),
        };

        let data = serde_json::to_value(CreateRoomRequest {
            max_clients: 2,
            known_port,
        })?;

        tx.send(Value::Object(json_map!{
            "type" => "room/create",
            "data" => data,
        }.into()))
            .await
            .unwrap_or_else(print_err);

        Ok(())
    }

    pub async fn join_room(&mut self, room_id: String) -> Result<(), IoOrSerdeError> {
        let tx = match &self.rendezvous_connection {
            Some(conn) => conn.tx.clone(),
            None => panic!(),
        };

        let data = serde_json::to_value(JoinRoomRequest {
            room_id,
        })?;

        tx.send(Value::Object(json_map!{
            "type" => "room/join",
            "data" => data,
        }.into()))
            .await
            .unwrap_or_else(print_err);

        Ok(())
    }
}
impl<'a> System<'a> for TransmissionNetworkPortal {
    type SystemData = ();
    
    fn run(&mut self, data: Self::SystemData) {

    }
}