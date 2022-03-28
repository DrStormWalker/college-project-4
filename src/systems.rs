use std::collections::{HashSet, VecDeque};
use nalgebra::max;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, WindowCanvas};
use specs::{AccessorCow, Entities, Join, ParJoin, Read, ReadStorage, RunningTime, System, Write, WriteExpect, WriteStorage};
use crate::components::{Collider, FloorCollider, FloorCollision, Physics, PlayerController, RenderDescriptor, Velocity};
use crate::{GameCamera, GameState, Grounded, Position, Rect, wchar_t, World};
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
        Read<'a, GameCamera>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        let (
            position,
            descriptor,
            camera,
        ) = data;

        for (pos, desc) in (&position, &descriptor).join() {
            self.canvas.set_draw_color(desc.colour());
            if let Some(rect) = camera.try_process_rect(pos.0, desc.rectangle()) {
                match self.canvas.fill_rect(rect) {
                    Err(e) => eprintln!("{}", e),
                    _ => {},
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
        WriteStorage<'a, Physics>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (
            mut position,
            mut velocity,
            mut physics,
            game_state,
        ) = data;

        let dt = game_state.delta_t;

        for physics in (&mut physics).join() {
            let f = physics.forces.iter().sum::<Vec2>();
            physics.acceleration = f / physics.mass;
        }

        for (vel, physics) in (&mut velocity, &physics).join() {
            vel.0 += physics.acceleration * dt;
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
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Grounded>,
        ReadStorage<'a, PlayerController>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (
            mut position,
            mut velocity,
            mut grounded,
            player_controlled,
            game_state
        ) = data;

        for (pos, vel, ground, _) in (
            &mut position,
            &mut velocity,
            &mut grounded,
            &player_controlled
        ).join() {
            let mut vx = 0.0f32;
            for key in &game_state.keys_held {
                use Keycode::*;
                match key {
                    A => vx += -8.0,
                    D => vx += 8.0,
                    W | Space if ground.0 => {
                        vel.0.y = 50.0;
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
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Collider>,
        WriteStorage<'a, Grounded>,
        ReadStorage<'a, FloorCollision>,
        ReadStorage<'a, FloorCollider>,
        Read<'a, GameState>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (
            mut position,
            velocity,
            physics,
            collider,
            grounded,
            floor_collision,
            floor_collider,
            game_state,
            entities,
        ) = data;

        let dt = game_state.delta_t;

        'objects: for (
            colliding,
            vel,
            physics,
            player_collider,
            ground,
            _,
        ) in (
            &entities,
            &velocity,
            &physics,
            &collider,
            &grounded,
            &floor_collision
        ).join() {
            'floors: for (floor, floor_collider, _) in (&entities, &collider, &floor_collider).join() {
                let floor_pos = if let Some(pos) = position.get(floor) { *pos } else { continue 'floors };
                let obj_pos = if let Some(pos) = position.get_mut(colliding) {
                    pos
                } else {
                    continue 'objects
                };

                let intersection = Collider::find_intersection(
                    player_collider.shape(),
                    obj_pos.0 + vel.0 * dt,
                    floor_collider.shape(),
                    floor_pos.0,
                );

                if let Some(intersection) = intersection {
                    println!("{:?} {:?}", obj_pos.0, intersection);
                    obj_pos.0 += intersection - vel.0 * dt;
                    physics.forces.ins
                }
            }
        }
    }
}
