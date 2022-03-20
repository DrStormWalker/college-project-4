use std::collections::HashSet;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, WindowCanvas};
use specs::{Join, ParJoin, Read, ReadStorage, System, Write, WriteStorage};
use crate::components::{Acceleration, PlayerController, RenderDescriptor, Velocity, VelocityLimit};
use crate::{GameState, Grounded, Position, wchar_t};
use crate::resources::SystemState;

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
            println!("{:?}", pos);

            self.canvas.set_draw_color(desc.colour());
            let mut rect = desc.rectangle().clone();
            let (x, y) = (rect.x(), rect.y());
            rect.set_x(x + (pos.x * 4.0) as i32);
            rect.set_y(y + (pos.y * 4.0) as i32);
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
        ReadStorage<'a, VelocityLimit>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (
            mut position,
            mut velocity,
            mut acceleration,
            velocity_limit,
            game_state,
        ) = data;

        let dt = game_state.delta_t;

        for (vel, accel) in (&mut velocity, &acceleration).join() {
            vel.x += accel.x * dt;
            vel.y += accel.y * dt;
        }

        for (vel, accel, vel_lim) in (&mut velocity, &mut acceleration, &velocity_limit).join() {
            if vel.x > vel_lim.x.end {
                accel.x = 0.0;
                vel.x = vel_lim.x.end;
            }

            if vel.x < vel_lim.x.start {
                accel.x = 0.0;
                vel.x = vel_lim.x.start;
            }

            if vel.y > vel_lim.y.end {
                accel.y = 0.0;
                vel.y = vel_lim.y.end;
            }

            if vel.y < vel_lim.y.start {
                accel.y = 0.0;
                vel.y = vel_lim.y.start;
            }
        }

        for (pos, vel) in (&mut position, &velocity).join() {
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;
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
            if pos.y >= 120.0 {
                ground.0 = true;
            }
            if pos.y + vel.y * game_state.delta_t > 120.0 {
                vel.y = 0.0;
                accel.y = 0.0;
                pos.y = 120.0;
            }
            let mut vx = 0.0f32;
            for key in &game_state.keys_held {
                use Keycode::*;
                match key {
                    A => vx += -80.0,
                    D => vx += 80.0,
                    W | Space if ground.0 => {
                        vel.y = -360.0;
                        accel.y = 960.0 * 1.5;
                        ground.0 = false;
                    }
                    _ => {},
                }
            }
            vel.x = vx;
        }
    }
}
