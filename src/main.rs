mod components;
mod resources;
mod systems;
mod util;
// mod gjk; // Not working
mod game;
mod networking;
mod sat;

extern crate sdl2;
extern crate serde;
extern crate specs;

use crate::networking::systems::TransmissionNetworkPortal;
use clap::{ArgEnum, Parser};
use std::net::{IpAddr, SocketAddr};

#[derive(Copy, Clone, ArgEnum, Debug)]
pub enum NetworkMode {
    None,
    Host,
    Client,
}

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[clap(arg_enum, short, long, default_value = "none")]
    pub networking: NetworkMode,

    #[clap(short, long, default_value = "127.0.0.1")]
    pub source: IpAddr,

    #[clap(long, default_value = "0")]
    pub send_port: u16,

    #[clap(long, default_value = "0")]
    pub recv_port: u16,

    #[clap(short, long, default_value = "127.0.0.1:50000")]
    pub rendezvous: SocketAddr,

    #[clap(short = 'i', long)]
    pub room_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();

    println!("{:?}", args);

    let portal = TransmissionNetworkPortal::new();

    if let NetworkMode::None = args.networking {
    } else {
        let portal = portal
            .rendezvous_init(args.rendezvous)
            .await
            .map_err(|e| e.to_string())?;

        match args.networking {
            NetworkMode::Host => portal
                .lock()
                .await
                .create_room(args.source, args.send_port, args.recv_port)
                .await
                .map_err(|e| e.to_string())?,
            NetworkMode::Client => {
                if let Some(ref id) = args.room_id {
                    portal
                        .lock()
                        .await
                        .join_room(id.clone(), args.source, args.send_port, args.recv_port)
                        .await
                        .map_err(|e| e.to_string())?
                } else {
                    panic!("Failed to supply a room id!")
                }
            }
            _ => {}
        }
    }

    game::game_main(args)
}
