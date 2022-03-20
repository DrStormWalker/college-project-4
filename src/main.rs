mod components;
mod systems;
mod resources;

extern crate sdl2;
extern crate specs;

use std::borrow::Borrow;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::{RTF_REINSTATE, wchar_t};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use specs::{ Builder, DispatcherBuilder, World, WorldExt };
use crate::{ components::Position, systems::RenderSystem };
use crate::components::{Acceleration, Grounded, PlayerController, RenderDescriptor, Velocity, VelocityLimit};
use crate::resources::{GameState, SystemState};
use crate::systems::{PlayerMovementSystem, EntityMovementSystem, EventSystem};

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

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump()?;

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerMovementSystem {}, "sys_player_movement", &[])
        .with(EntityMovementSystem {}, "sys_entity_movement", &[])
        .with_thread_local(EventSystem::new(event_pump))
        .with_thread_local(RenderSystem::new(canvas))
        .build();

    dispatcher.setup(&mut world);

    world.insert(GameState::new(SystemState::Running));

    world
        .create_entity()
        .with(Position { x: 0.0, y: 120.0 })
        .with(Velocity { x: 0.0, y: 0.0 })
        //.with(VelocityLimit { x: f32::MIN..f32::MAX, y: -4.0..f32::MAX })
        .with(Acceleration { x: 0.0, y: 0.0 })
        .with(RenderDescriptor::new(
            Rect::new(0, 0, 20, 20),
            Color::RGB(255, 0, 0))
        )
        .with(PlayerController {})
        .with(Grounded(true))
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