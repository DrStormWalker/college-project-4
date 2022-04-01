use std::collections::HashMap;
use std::ops::{Range, RangeBounds, RangeFull};
use sdl2::libc::wait;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use specs::{Component, Entity, System, VecStorage};
use crate::util::{Polygon, Rect};
use crate::{Vec2, wchar_t};

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

#[derive(Debug, PartialEq)]
pub struct Acceleration(pub Vec2);
impl Component for Acceleration {
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
            shape: Polygon::new(rect.vertices().to_vec()),
        }
    }

    pub fn shape(&self) -> &Polygon {
        &self.shape
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
