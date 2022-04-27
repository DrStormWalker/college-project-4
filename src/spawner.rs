use sdl2::pixels::Color;
use specs::{Builder, World, WorldExt};

use crate::{
    components::{
        Acceleration, Collider, FloorCollider, FloorCollision, Grounded, PlayerController,
        Position, RenderDescriptor, Velocity,
    },
    networking::components::{NetworkRecv, NetworkSend, NetworkStatic},
    util::{Rect, Vec2},
};

const PLAYER_WIDTH: f32 = 1.0;
const PLAYER_HEIGHT: f32 = 1.0;

const ACCELERATION_DUE_TO_GRAVITY: f32 = -130.0;

pub fn create_player(world: &mut World, id: usize, pos: Vec2) {
    world
        .create_entity()
        .with(Position(pos))
        .with(Velocity(Vec2::new(0.0, 0.0)))
        .with(Acceleration(Vec2::new(0.0, ACCELERATION_DUE_TO_GRAVITY)))
        .with(RenderDescriptor::new(
            Rect::from_centre(Vec2::new(0.0, 0.0), PLAYER_WIDTH, PLAYER_HEIGHT),
            Color::RGB(255, 0, 0),
        ))
        .with(Grounded(true))
        .with(PlayerController {})
        .with(Collider::new(Rect::from_centre(
            Vec2::new(0.0, 0.0),
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        )))
        .with(FloorCollision {})
        .with(NetworkSend::new(id))
        .build();
}

pub fn create_peer_player(world: &mut World, id: usize, pos: Vec2) {
    world
        .create_entity()
        .with(Position(pos))
        .with(Velocity(Vec2::new(0.0, 0.0)))
        .with(Acceleration(Vec2::new(0.0, ACCELERATION_DUE_TO_GRAVITY)))
        .with(RenderDescriptor::new(
            Rect::from_centre(Vec2::new(0.0, 0.0), PLAYER_WIDTH, PLAYER_HEIGHT),
            Color::RGB(0, 0, 255),
        ))
        .with(Collider::new(Rect::from_centre(
            Vec2::new(0.0, 0.0),
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        )))
        .with(FloorCollision {})
        .with(NetworkRecv::new(id))
        .build();
}

pub fn create_wall(world: &mut World, id: usize, pos: Vec2, width: f32, height: f32) {
    world
        .create_entity()
        .with(Position(pos))
        .with(RenderDescriptor::new(
            Rect::from_size(Vec2::new(0.0, 0.0), width, height),
            Color::RGB(0, 255, 0),
        ))
        .with(Collider::new(Rect::from_size(
            Vec2::new(0.0, 0.0),
            width,
            height,
        )))
        .with(FloorCollider {})
        .with(NetworkStatic::new(id))
        .build();
}
