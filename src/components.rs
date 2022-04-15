use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Range, RangeBounds, RangeFull};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use specs::{Component, Entity, System, VecStorage};
use crate::util::{Polygon, Rect, Shape2D, Vec2};

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

pub struct Collider<'a> {
    shape: Box<dyn Shape2D + Send + Sync + 'a>,
}
impl<'a> Collider<'a> {
    pub fn new(shape: impl Shape2D + Send + Sync + 'a) -> Self {
        Self { shape: Box::new(shape) }
    }

    pub fn shape(&self) -> &Box<dyn Shape2D + Send + Sync + 'a> {
        &self.shape
    }
}
impl Component for Collider<'static> {
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
