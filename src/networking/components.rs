use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};
use specs::{Component, Join, ReadStorage, System, VecStorage, WriteStorage};
use tokio::sync::{broadcast, mpsc, Mutex};

use crate::{
    components::{Acceleration, Position, Velocity},
    util::Vec2,
};

use super::systems::{Message, RoomConnectionType, TransmissionNetworkPortal};

pub struct Incrementor {
    value: usize,
}
impl Incrementor {
    pub fn new() -> Self {
        Self { value: 1 }
    }
}
impl Iterator for Incrementor {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.value;
        self.value += 1;
        Some(current)
    }
}

#[derive(Debug, PartialEq)]
pub struct NetworkSend {
    network_id: usize,
}
impl NetworkSend {
    pub fn new(network_id: usize) -> Self {
        Self { network_id }
    }
}
impl Component for NetworkSend {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, PartialEq)]
pub struct NetworkRecv {
    network_id: usize,
}
impl NetworkRecv {
    pub fn new(network_id: usize) -> Self {
        Self { network_id }
    }
}
impl Component for NetworkRecv {
    type Storage = VecStorage<Self>;
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Vec2ForSerde {
    pub x: f32,
    pub y: f32,
}
impl From<Vec2> for Vec2ForSerde {
    fn from(vec: Vec2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}
impl Into<Vec2> for Vec2ForSerde {
    fn into(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateEntity {
    entity_id: usize,
    position: Vec2ForSerde,
    velocity: Vec2ForSerde,
    acceleration: Vec2ForSerde,
}
impl UpdateEntity {
    pub fn new(entity_id: usize, position: Vec2, velocity: Vec2, acceleration: Vec2) -> Self {
        Self {
            entity_id,
            position: position.into(),
            velocity: velocity.into(),
            acceleration: acceleration.into(),
        }
    }
}

pub struct NetworkHandler {
    portal: Arc<Mutex<TransmissionNetworkPortal>>,
    channels: (broadcast::Sender<Message>, mpsc::Receiver<Message>),
}
impl NetworkHandler {
    pub fn new(
        portal: Arc<Mutex<TransmissionNetworkPortal>>,
        channels: (broadcast::Sender<Message>, mpsc::Receiver<Message>),
    ) -> Self {
        Self { portal, channels }
    }
}
impl<'a> System<'a> for NetworkHandler {
    type SystemData = (
        ReadStorage<'a, NetworkSend>,
        ReadStorage<'a, NetworkRecv>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Acceleration>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (network_send, network_recv, mut position, mut velocity, mut acceleration) = data;

        for (position, velocity, acceleration, network_send) in
            (&position, &velocity, &acceleration, &network_send).join()
        {
            let msg = Message::new(
                "entity/update".to_string(),
                UpdateEntity::new(
                    network_send.network_id,
                    position.0,
                    velocity.0,
                    acceleration.0,
                ),
            );

            self.channels.0.send(msg);
        }

        while let Ok(msg) = self.channels.1.try_recv() {
            let msg: UpdateEntity = serde_json::from_value(msg.data).unwrap();
            for (mut position, mut velocity, mut acceleration, network_recv) in (
                &mut position,
                &mut velocity,
                &mut acceleration,
                &network_recv,
            )
                .join()
                .filter(|c| c.3.network_id == msg.entity_id)
            {
                position.0 = msg.position.into();
                velocity.0 = msg.velocity.into();
                acceleration.0 = msg.acceleration.into();
            }
        }
    }
}
