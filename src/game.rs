use std::sync::Arc;
use std::time::{Duration, Instant};

use sdl2::pixels::Color;
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio::time;

use crate::components::{
    Acceleration, Collider, FloorCollider, FloorCollision, Grounded, PlayerController,
    RenderDescriptor, Velocity,
};
use crate::networking::components::{NetworkHandler, NetworkRecv, NetworkSend, NetworkStatic};
use crate::networking::systems::{Message, TransmissionNetworkPortal};
use crate::resources::{GameCamera, GameState, SystemState};
use crate::spawner;
use crate::systems::{
    EntityMovementSystem, EventSystem, FloorColliderSystem, PlayerDebug, PlayerMovementSystem,
};
use crate::util::{Incrementor, Rect, Vec2};
use crate::NetworkMode;
use crate::{components::Position, systems::RenderSystem, Args};
use specs::{Builder, DispatcherBuilder, World, WorldExt};

pub async fn game_main(
    args: Args,
    portal: Arc<Mutex<TransmissionNetworkPortal>>,
    channels: (broadcast::Sender<Message>, mpsc::Receiver<Message>),
) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Team Platformer", 800, 600)
        .position_centered()
        .vulkan()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut world = World::new();

    world.register::<FloorCollision>();
    world.register::<FloorCollider>();
    world.register::<NetworkStatic>();
    world.register::<NetworkSend>();
    world.register::<NetworkRecv>();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump()?;

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerDebug {}, "sys_player_debug", &[])
        .with(PlayerMovementSystem {}, "sys_player_movement", &[])
        .with(EntityMovementSystem {}, "sys_entity_movement", &[])
        .with(
            FloorColliderSystem {},
            "sys_floor_collision",
            &["sys_entity_movement"],
        )
        .with(
            NetworkHandler::new(portal, channels),
            "sys_network_handler",
            &["sys_entity_movement"],
        )
        .with_thread_local(EventSystem::new(event_pump))
        .with_thread_local(RenderSystem::new(canvas))
        .build();

    dispatcher.setup(&mut world);

    world.insert(GameState::new(SystemState::Running));
    world.insert(GameCamera::new(
        (800, 600),
        Vec2::new(0.0, 0.0),
        Rect::new(Vec2::new(-16.0, 12.0), Vec2::new(16.0, -12.0)),
    ));

    // Spawn objects

    let mut ids = Incrementor::new();

    spawner::create_player(&mut world, ids.next().unwrap(), Vec2::new(0.0, 0.0));

    // Floor
    spawner::create_wall(
        &mut world,
        ids.next().unwrap(),
        Vec2::new(-16.0, -10.0),
        32.0,
        4.0,
    );

    // Platform
    spawner::create_wall(
        &mut world,
        ids.next().unwrap(),
        Vec2::new(-10.0, -5.0),
        8.0,
        1.0,
    );

    // Left wall
    spawner::create_wall(
        &mut world,
        ids.next().unwrap(),
        Vec2::new(-16.5, 12.0),
        0.5,
        24.0,
    );

    // Right wall
    spawner::create_wall(
        &mut world,
        ids.next().unwrap(),
        Vec2::new(16.0, 12.0),
        0.5,
        24.0,
    );

    // Start game loop

    let mut start = Instant::now();

    let frame_interval = Duration::from_secs_f64(1.0 / 120.0);
    let mut frame_interval = time::interval(frame_interval);
    frame_interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        frame_interval.tick().await;
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
    }

    Ok(())
}
