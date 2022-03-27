use std::collections::{HashSet, VecDeque};
use nalgebra::max;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, WindowCanvas};
use specs::{AccessorCow, Join, ParJoin, Read, ReadStorage, RunningTime, System, Write, WriteExpect, WriteStorage};
use crate::components::{Acceleration, Collider, FloorCollider, FloorCollision, PlayerController, RenderDescriptor, Velocity};
use crate::{GameState, Grounded, Position, Rect, wchar_t, World};
use crate::resources::SystemState;
use crate::util::Vec2;

pub struct RenderSystem {
    canvas: WindowCanvas,
}
impl RenderSystem {
    pub fn new(canvas: WindowCanvas) -> Self {
        Self { canvas }
    }
}
impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, RenderDescriptor>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        let (position, descriptor) = data;
        for (pos, desc) in (&position, &descriptor).join() {
            self.canvas.set_draw_color(desc.colour());
            let rect = desc.rectangle()
                .enlarged(Vec2::new(4.0, 4.0))
                .shifted(pos.0 * 4.0)
                .into_sdl2_rect();
            match self.canvas.fill_rect(rect) {
                Err(e) => eprintln!("{}", e),
                _ => {},
            }
        }
        self.canvas.present();
    }
}

pub struct EventSystem {
    event_pump: EventPump,
}
impl EventSystem {
    pub fn new (event_pump: EventPump) -> Self {
        Self { event_pump }
    }
}
impl<'a> System<'a> for EventSystem {
    type SystemData = Write<'a, GameState>;

    fn run(&mut self, mut data: Self::SystemData) {
        let mut game_state = data;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => game_state.system_state = SystemState::Quit,
                _ => {},
            }
        }

        let keys = self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect::<HashSet<Keycode>>();

        game_state.keys_pressed = &keys - &game_state.keys_held;
        game_state.keys_released = &game_state.keys_held - &keys;
        game_state.keys_held = keys;
    }
}

pub struct EntityMovementSystem;
impl<'a> System<'a> for EntityMovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Acceleration>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (
            mut position,
            mut velocity,
            mut acceleration,
            game_state,
        ) = data;

        let dt = game_state.delta_t;

        for (vel, accel) in (&mut velocity, &acceleration).join() {
            vel.0 += accel.0 * dt;
        }

        for (pos, vel) in (&mut position, &velocity).join() {
            pos.0 += vel.0 * dt;
        }
    }
}

pub struct PlayerMovementSystem;
impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Acceleration>,
        WriteStorage<'a, Grounded>,
        ReadStorage<'a, PlayerController>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (
            mut position,
            mut velocity,
            mut acceleration,
            mut grounded,
            player_controlled,
            game_state
        ) = data;

        for (pos, vel, accel, ground, _) in (
            &mut position,
            &mut velocity,
            &mut acceleration,
            &mut grounded,
            &player_controlled
        ).join() {
            let mut vx = 0.0f32;
            for key in &game_state.keys_held {
                use Keycode::*;
                match key {
                    A => vx += -80.0,
                    D => vx += 80.0,
                    W | Space if ground.0 => {
                        vel.0.y = -360.0;
                        accel.0.y = 960.0 * 1.8;
                        ground.0 = false;
                    }
                    _ => {},
                }
            }
            vel.0.x = vx;
        }
    }
}

pub struct FloorColliderSystem;
impl<'a> System<'a> for FloorColliderSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Acceleration>,
        WriteStorage<'a, Collider>,
        WriteStorage<'a, Grounded>,
        ReadStorage<'a, FloorCollision>,
        ReadStorage<'a, FloorCollider>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (
            position,
            velocity,
            acceleration,
            collider,
            grounded,
            floor_collision,
            floor_collider,
            game_state,
        ) = data;

        let dt = game_state.delta_t;

        for (
            pos,
            vel,
            accel,
            player_collider,
            ground,
            _,
        ) in (
            &position,
            &velocity,
            &acceleration,
            &collider,
            &grounded,
            &floor_collision
        ).join() {
            for (floor_pos, floor_collider, _) in (&position, &collider, &floor_collider).join() {
                let intersecting = Collider::test_collision(
                    player_collider.shape(),
                    pos.0,
                    floor_collider.shape(),
                    floor_pos.0,
                );

                if intersecting {
                    println!("Intersection {}", intersecting);
                }
            }
        }
    }
}
