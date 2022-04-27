use std::collections::HashSet;

use crate::components::{
    Acceleration, Collider, FloorCollider, FloorCollision, Grounded, PlayerController, Position,
    RenderDescriptor, Velocity,
};
use crate::resources::{GameCamera, GameState, SystemState};
use crate::sat::intersection;
use crate::util::Vec2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

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
        Read<'a, GameCamera>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        let (position, descriptor, camera) = data;

        for (pos, desc) in (&position, &descriptor).join() {
            self.canvas.set_draw_color(desc.colour());
            if let Some(rect) = camera.try_process_rect(pos.0, desc.rectangle()) {
                match self.canvas.fill_rect(rect) {
                    Err(e) => eprintln!("{}", e),
                    _ => {}
                }
            }
        }
        self.canvas.present();
    }
}

pub struct EventSystem {
    event_pump: EventPump,
}
impl EventSystem {
    pub fn new(event_pump: EventPump) -> Self {
        Self { event_pump }
    }
}
impl<'a> System<'a> for EventSystem {
    type SystemData = Write<'a, GameState>;

    fn run(&mut self, data: Self::SystemData) {
        let mut game_state = data;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => game_state.system_state = SystemState::Quit,
                _ => {}
            }
        }

        let keys = self
            .event_pump
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
        WriteStorage<'a, Grounded>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (mut position, mut velocity, mut accel, mut grounded, game_state) = data;

        let dt = game_state.delta_t;

        for accel in (&mut accel).join() {
            accel.0.y = -130.0;
        }

        for ground in (&mut grounded).join() {
            ground.0 = false;
        }

        for (vel, accel) in (&mut velocity, &accel).join() {
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
        WriteStorage<'a, Grounded>,
        ReadStorage<'a, PlayerController>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (mut position, mut velocity, mut grounded, player_controlled, game_state) = data;

        for (_pos, vel, ground, _) in (
            &mut position,
            &mut velocity,
            &mut grounded,
            &player_controlled,
        )
            .join()
        {
            let mut vx = 0.0f32;
            for key in &game_state.keys_held {
                use Keycode::*;
                match key {
                    A => vx += -12.0,
                    D => vx += 12.0,
                    W | Space if ground.0 => {
                        vel.0.y = 40.0;
                        ground.0 = false;
                    }
                    _ => {}
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
        WriteStorage<'a, Collider<'static>>,
        WriteStorage<'a, Grounded>,
        ReadStorage<'a, FloorCollision>,
        ReadStorage<'a, FloorCollider>,
        Read<'a, GameState>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut position,
            mut velocity,
            mut acceleration,
            collider,
            mut grounded,
            floor_collision,
            floor_collider,
            game_state,
            entities,
        ) = data;

        let _dt = game_state.delta_t;

        'objects: for (colliding, vel, accel, player_collider, mut ground, _) in (
            &entities,
            &mut velocity,
            &mut acceleration,
            &collider,
            (&mut grounded).maybe(),
            &floor_collision,
        )
            .join()
        {
            'floors: for (floor, floor_collider, _) in
                (&entities, &collider, &floor_collider).join()
            {
                let floor_pos = if let Some(pos) = position.get(floor) {
                    *pos
                } else {
                    continue 'floors;
                };
                let obj_pos = if let Some(pos) = position.get_mut(colliding) {
                    pos
                } else {
                    continue 'objects;
                };

                let intersection = intersection(
                    player_collider.shape(),
                    obj_pos.0,
                    floor_collider.shape(),
                    floor_pos.0,
                );

                if let Some(n) = intersection {
                    if n.magnitude() != 0.0 {
                        //let n = -n;
                        let norm_n = n.normalize();
                        let new_vel = norm_n * vel.0.dot(&norm_n);
                        let new_accel = norm_n * accel.0.dot(&norm_n);
                        let norm = obj_pos.0 - floor_pos.0;
                        let norm = norm_n * norm.dot(&norm_n);
                        let norm = norm.normalize();

                        println!("norm: {:?}", norm);

                        vel.0 -= new_vel;
                        accel.0 -= new_accel;

                        obj_pos.0 += norm * n.magnitude();

                        use std::f32::consts::FRAC_1_SQRT_2;

                        if let Some(ground) = &mut ground {
                            if Vec2::y_axis().dot(&norm.normalize()) > FRAC_1_SQRT_2 {
                                ground.0 = true;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct PlayerDebug;
impl<'a> System<'a> for PlayerDebug {
    type SystemData = (ReadStorage<'a, PlayerController>, ReadStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (player, position) = data;

        for (position, _) in (&position, &player).join() {
            println!("{:?}", position);
        }
    }
}
