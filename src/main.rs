mod components;
mod systems;
mod resources;
mod util;
// mod gjk; // Not working
mod sat;
mod networking;
mod game;

extern crate serde;
extern crate sdl2;
extern crate specs;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use clap::{Parser, ArgEnum};
use crate::networking::systems::TransmissionNetworkPortal;

#[derive(Copy, Clone, ArgEnum, Debug)]
pub enum NetworkMode {
    None,
    Host,
    Client,
}

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(arg_enum, short, long, default_value = "none")]
    pub networking: NetworkMode,

    #[clap(short, long, default_value = "127.0.0.1")]
    pub source: IpAddr,

    #[clap(short, long, default_value = "50001")]
    pub port: u16,

    #[clap(long, default_value = "50002")]
    pub known_port: u16,

    #[clap(short, long, default_value = "127.0.0.1:50000")]
    pub rendezvous: SocketAddr,

    #[clap(short = 'i', long)]
    pub room_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();

    println!("{:?}", args);

    let mut portal = TransmissionNetworkPortal::new();

    if let NetworkMode::None = args.networking {}
    else {
        portal.rendezvous_init(args.rendezvous)
            .await
            .map_err(|e| e.to_string())?;

        match args.networking {
            NetworkMode::Host => portal.create_room(args.known_port).await
                .map_err(|e| e.to_string())?,
            NetworkMode::Client => {
                if let Some(ref id) = args.room_id {
                    portal.join_room(id.clone()).await
                        .map_err(|e| e.to_string())?
                } else {
                    panic!("Failed to supply a room id!")
                }
            },
            _ => {},
        }
    }

    game::game_main(args)
}