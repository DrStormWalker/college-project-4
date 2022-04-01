mod components;
mod systems;
mod resources;
mod util;
// mod gjk; // Not working
mod sat;
mod networking;

extern crate sdl2;
extern crate specs;

use std::borrow::Borrow;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::{RTF_REINSTATE, wchar_t};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use specs::{ Builder, DispatcherBuilder, World, WorldExt };
use crate::{ components::Position, systems::RenderSystem };
use crate::components::{Acceleration, Collider, FloorCollider, FloorCollision, Grounded, PlayerController, RenderDescriptor, Velocity};
use crate::resources::{GameCamera, GameState, SystemState};
use crate::systems::{PlayerMovementSystem, EntityMovementSystem, EventSystem, FloorColliderSystem};
use crate::util::{Rect, Vec2};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Team Platformer", 800, 600)
        .position_centered()
        .vulkan()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let mut world = World::new();

    world.register::<FloorCollision>();
    world.register::<FloorCollider>();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump()?;

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerMovementSystem {}, "sys_player_movement", &[])
        .with(EntityMovementSystem {}, "sys_entity_movement", &[])
        .with(FloorColliderSystem {}, "sys_floor_collision", &["sys_entity_movement"])
        .with_thread_local(EventSystem::new(event_pump))
        .with_thread_local(RenderSystem::new(canvas))
        .build();

    dispatcher.setup(&mut world);

    world.insert(GameState::new(SystemState::Running));
    world.insert(GameCamera::new(
        (800, 600),
        Vec2::new(0.0, 0.0),
        Rect::new(
            Vec2::new(-16.0, 12.0),
            Vec2::new(16.0, -12.0),
        ),
    ));

    const PLAYER_WIDTH: f32 = 1.0;
    const PLAYER_HEIGHT: f32 = 1.0;

    world
        .create_entity()
        .with(Position(Vec2::new(0.0, 0.0)))
        .with(Velocity(Vec2::new(0.0, 0.0)))
        .with(Acceleration(Vec2::new(0.0, -130.0)))
        .with(RenderDescriptor::new(
            Rect::from_centre(Vec2::new(0.0, 0.0), PLAYER_WIDTH, PLAYER_HEIGHT),
            Color::RGB(255, 0, 0))
        )
        .with(Grounded(true))
        .with(PlayerController {})
        .with(Collider::from_rect(Rect::from_centre(Vec2::new(0.0, 0.0), PLAYER_WIDTH, PLAYER_HEIGHT)))
        .with(FloorCollision {})
        .build();

    world
        .create_entity()
        .with(Position(Vec2::new(-16.0, -10.0)))
        .with(RenderDescriptor::new(
            Rect::from_size(Vec2::new(0.0, 0.0), 32.0, 2.0),
            Color::RGB(0, 255, 0),
        ))
        .with(Collider::from_rect(Rect::from_size(Vec2::new(0.0, 0.0), 32.0, 2.0)))
        .with(FloorCollider {})
        .build();

    world
        .create_entity()
        .with(Position(Vec2::new(-10.0, -5.0)))
        .with(RenderDescriptor::new(
            Rect::from_size(Vec2::new(0.0, 0.0), 8.0, 1.0),
            Color::RGB(0, 255, 0),
        ))
        .with(Collider::from_rect(Rect::from_size(Vec2::new(0.0, 0.0), 8.0, 1.0)))
        .with(FloorCollider {})
        .build();

    let mut start = Instant::now();

    loop {
        let now = Instant::now();
        let delta_t = (now - start).as_secs_f32();
        start = now;

        {
            let mut game_state = world.write_resource::<GameState>();
            game_state.delta_t = delta_t;
        }

        dispatcher.dispatch(&mut world);
        {
            let game_state = world.read_resource::<GameState>();
            if let SystemState::Quit = game_state.system_state {
                break;
            }
        }
        world.maintain();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }

    Ok(())
}