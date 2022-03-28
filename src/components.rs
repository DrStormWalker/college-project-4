use std::collections::HashMap;
use std::ops::{Range, RangeBounds, RangeFull};
use sdl2::libc::wait;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use specs::{Component, Entity, System, VecStorage};
use crate::util::{Polygon, Rect};
use crate::{Vec2, wchar_t};
use crate::gjk::GJK;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position(pub Vec2);
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, PartialEq)]
pub struct Velocity(pub Vec2);
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

pub const GAME_GRAVITATIONAL_CONSTANT: f32 = -190.0;

#[derive(PartialEq, Hash)]
pub enum ForceSource {
    Constant(usize),
    Entity(Entity),
}

#[derive(Debug, PartialEq)]
pub struct Physics {
    pub acceleration: Vec2,
    pub forces: HashMap<ForceSource, Vec2>,
    pub mass: usize,
}
impl Physics {
    pub fn new(mass: usize) -> Self {
        let mut forces = HashMap::new();
        forces.insert(ForceSource::Constant(0), Vec2::new(0.0, mass * GAME_GRAVITATIONAL_CONSTANT));
        Self {
            acceleration: Vec2::zeros(),
            mass,
            forces,
        }
    }

    pub fn acceleration(&self) -> Vec2 {
        self.acceleration
    }
}
impl Component for Physics {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, PartialEq)]
pub struct Grounded(pub bool);
impl Component for Grounded {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, PartialEq)]
pub struct RenderDescriptor {
    rectangle: Rect,
    colour: Color,
}
impl RenderDescriptor {
    pub fn new(rectangle: Rect, colour: Color) -> Self {
        Self { rectangle, colour }
    }

    pub fn rectangle(&self) -> Rect {
        self.rectangle
    }

    pub fn colour(&self) -> Color {
        self.colour
    }
}
impl Component for RenderDescriptor {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, PartialEq)]
pub struct PlayerController;
impl Component for PlayerController {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, PartialEq)]
pub struct Collider {
    shape: Polygon,
}
impl Collider {
    pub fn new(shape: Polygon) -> Self {
        Self { shape }
    }

    pub fn from_rect(rect: Rect) -> Self {
        Self {
            shape: Polygon::new(vec![
                rect.top_left(),
                rect.top_right(),
                rect.bottom_right(),
                rect.bottom_left(),
            ]),
        }
    }

    pub fn shape(&self) -> &Polygon {
        &self.shape
    }

    pub fn find_intersection(a: &Polygon, a_pos: Vec2, b: &Polygon, b_pos: Vec2) -> Option<Vec2> {
        use crate::gjk::GJK;

        let mut gjk = GJK::new(a, a_pos, b, b_pos);

        gjk.find_intersection()
    }
}
impl Component for Collider {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, PartialEq)]
pub struct FloorCollider;
impl Component for FloorCollider {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, PartialEq)]
pub struct FloorCollision;
impl Component for FloorCollision {
    type Storage = VecStorage<Self>;
}
