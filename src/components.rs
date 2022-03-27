use std::ops::{Range, RangeBounds, RangeFull};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use specs::{Component, System, VecStorage };
use crate::util::{Polygon, Rect};
use crate::{Vec2, wchar_t};
use crate::components::gjk::gjk_test_collision;

#[derive(Debug, PartialEq)]
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

mod gjk {
    /* https://blog.hamaluik.ca/posts/building-a-collision-engine-part-2-2d-penetration-vectors/ */
    use std::collections::VecDeque;
    use nalgebra::Vector3;
    use crate::util::{Polygon};
    use crate::{Vec2, wchar_t};

    struct CollisionObject<T> {
        pos: Vec2,
        collider: T,
    }

    #[derive(PartialEq)]
    enum EvolutionResult {
        NoIntersection,
        FoundIntersection,
        StillEvolving,
    }

    fn add_support(
        a: &Polygon,
        a_pos: Vec2,
        b: &Polygon,
        b_pos: Vec2,
        vertices: &mut Vec<Vec2>,
        direction: Vec2,
    ) -> bool {
        let vertex = (a.support(direction) + a_pos) - (b.support(-direction) + b_pos);
        vertices.push(vertex);
        direction.dot(&vertex) > 0.0
    }

    fn triple_product(a: Vec2, b: Vec2, c: Vec2) -> Vec2 {
        Vector3::<f32>::new(a.x, a.y, 0.0)
            .cross(&Vector3::<f32>::new(b.x, b.y, 0.0))
            .cross(&Vector3::<f32>::new(c.x, c.y, 0.0))
            .xy()
    }

    fn evolve_simplex(
        a: &Polygon,
        a_pos: Vec2,
        b: &Polygon,
        b_pos: Vec2,
        vertices: &mut Vec<Vec2>,
        direction: &mut Vec2
    ) -> EvolutionResult {
        match vertices.len() {
            0 => {
                *direction = b_pos - a_pos;
            },
            1 => {
                *direction = -*direction;
            }
            2 => {
                let a = vertices[1];
                let b = vertices[0];

                let ab = b - a;
                let oa = -a;

                *direction = triple_product(ab, oa, ab);
            },
            3 => {
                let a = vertices[2];
                let b = vertices[1];
                let c = vertices[0];

                let ab = b - a;
                let ac = c - a;
                let oa = -a;

                let ab_perpendicular = triple_product(ac, ab, ab);
                let ac_perpendicular = triple_product(ab, ac, ac);

                if ab_perpendicular.dot(&oa) > 0.0 {
                    vertices.remove(0);
                    *direction = ab_perpendicular;
                } else if ac_perpendicular.dot(&oa) > 0.0 {
                    vertices.remove(1);
                    *direction = ac_perpendicular;
                } else {
                    return EvolutionResult::FoundIntersection;
                }
            },
            _ => panic!("BUG ALERT! GJK")
        }

        if add_support(a, a_pos, b, b_pos, vertices, *direction) {
            EvolutionResult::StillEvolving
        } else {
            EvolutionResult::NoIntersection
        }
    }

    pub fn gjk_test_collision(a: &Polygon, a_pos: Vec2, b: &Polygon, b_pos: Vec2) -> bool {
        let mut vertices = Vec::with_capacity(3);
        let mut direction = Vec2::default();

        let mut result = EvolutionResult::StillEvolving;
        while let EvolutionResult::StillEvolving = result {
            result = evolve_simplex(a, a_pos, b, b_pos, &mut vertices, &mut direction)
        }

        result == EvolutionResult::FoundIntersection
    }
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

    pub fn test_collision(a: &Polygon, a_pos: Vec2, b: &Polygon, b_pos: Vec2) -> bool {
        gjk_test_collision(a, a_pos, b, b_pos)
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
