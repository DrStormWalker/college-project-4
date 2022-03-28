use std::collections::VecDeque;
use nalgebra::Vector3;
use sdl2::version::version;
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

#[derive(Copy, Clone)]
enum PolygonWinding {
    Clockwise,
    AntiClockwise,
}

struct Edge {
    distance: f32,
    normal: Vec2,
    index: usize,
}
impl Edge {
    pub fn new(distance: f32, normal: Vec2, index: usize) -> Self {
        Self {
            distance,
            normal,
            index,
        }
    }
}

fn triple_product(a: Vec2, b: Vec2, c: Vec2) -> Vec2 {
    Vector3::<f32>::new(a.x, a.y, 0.0)
        .cross(&Vector3::<f32>::new(b.x, b.y, 0.0))
        .cross(&Vector3::<f32>::new(c.x, c.y, 0.0))
        .xy()
}

const EPA_ITERATIONS: usize = 1;

pub struct GJK<'a> {
    shape_a: &'a Polygon,
    pos_a: Vec2,
    shape_b: &'a Polygon,
    pos_b: Vec2,
    vertices: Vec<Vec2>,
    direction: Vec2,
}
impl<'a> GJK<'a> {
    pub fn new(shape_a: &'a Polygon, pos_a: Vec2, shape_b: &'a Polygon, pos_b: Vec2) -> Self {
        Self {
            shape_a,
            pos_a,
            shape_b,
            pos_b,
            vertices: vec![],
            direction: Vec2::new(1.0, 0.0),
        }
    }

    fn add_support(&mut self) -> bool {
        let vertex = (self.shape_a.support(self.direction) + self.pos_a)
            - (self.shape_b.support(-self.direction) + self.pos_b);
        self.vertices.push(vertex);
        self.direction.dot(&vertex) > 0.0
    }

    fn evolve_simplex(&mut self) -> EvolutionResult {
        match self.vertices.len() {
            0 => {
                self.direction = self.pos_b - self.pos_a;
            },
            1 => {
                self.direction = -self.direction;
            }
            2 => {
                let a = self.vertices[1];
                let b = self.vertices[0];

                let ab = b - a;
                let oa = -a;

                self.direction = triple_product(ab, oa, ab);
            },
            3 => {
                let a = self.vertices[2];
                let b = self.vertices[1];
                let c = self.vertices[0];

                let ab = b - a;
                let ac = c - a;
                let oa = -a;

                let ab_perpendicular = triple_product(ac, ab, ab);
                let ac_perpendicular = triple_product(ab, ac, ac);

                if ab_perpendicular.dot(&oa) > 0.0 {
                    self.vertices.remove(0);
                    self.direction = ab_perpendicular;
                } else if ac_perpendicular.dot(&oa) > 0.0 {
                    self.vertices.remove(1);
                    self.direction = ac_perpendicular;
                } else {
                    return EvolutionResult::FoundIntersection;
                }
            },
            _ => panic!("BUG ALERT! GJK")
        }

        if self.add_support() {
            EvolutionResult::StillEvolving
        } else {
            EvolutionResult::NoIntersection
        }
    }

    pub fn test_collision(&mut self) -> bool {
        self.vertices = Vec::with_capacity(3);
        self.direction = Vec2::default();

        let mut result = EvolutionResult::StillEvolving;
        while let EvolutionResult::StillEvolving = result {
            result = self.evolve_simplex()
        }

        result == EvolutionResult::FoundIntersection
    }

    /* EPA (Expanding Polytope Algorithm)
     * based on:
     * https://blog.hamaluik.ca/posts/building-a-collision-engine-part-2-2d-penetration-vectors/
     */

    fn find_closest_edge(&mut self, winding: PolygonWinding) -> Edge {
        let mut closest_distance = f32::INFINITY;
        let mut closest_normal = Vec2::zeros();
        let mut closest_index = 0usize;

        for i in 0..self.vertices.len() {
            let j = if i + 1 >= self.vertices.len() { 0 } else { i + 1 };

            let line = self.vertices[j] - self.vertices[i];

            let normal = match winding {
                PolygonWinding::Clockwise => Vec2::new(-line.y, line.x),
                PolygonWinding::AntiClockwise => Vec2::new(line.y, -line.x),
            };
            let normal = normal.normalize();

            let dist = normal.dot(&self.vertices[i]);
            if dist < closest_distance {
                closest_distance = dist;
                closest_normal = normal;
                closest_index = j;
            }
        }

        Edge::new(closest_distance, closest_normal, closest_index)
    }

    fn calculate_support(&self, direction: Vec2) -> Vec2 {
        self.shape_a.support(direction) - self.shape_b.support(-direction)
    }

    pub fn find_intersection(&mut self) -> Option<Vec2> {
        if !self.test_collision() { return None }

        let e0 = (self.vertices[1].x - self.vertices[0].x) * (self.vertices[1].y + self.vertices[0].y);
        let e1 = (self.vertices[2].x - self.vertices[1].x) * (self.vertices[2].y + self.vertices[1].y);
        let e2 = (self.vertices[0].x - self.vertices[2].x) * (self.vertices[0].y + self.vertices[2].y);

        let winding = if e0 + e1 + e2 >= 0.0 { PolygonWinding::Clockwise } else { PolygonWinding::AntiClockwise };

        let mut intersection = Vec2::zeros();
        for i in 0..EPA_ITERATIONS {
            let edge = self.find_closest_edge(winding);
            let support = self.calculate_support(edge.normal);
            let distance = support.dot(&edge.normal);

            intersection = edge.normal * distance;

            if (distance - edge.distance).abs() <= 1e-6 {
                return Some(intersection)
            } else {
                self.vertices.insert(edge.index, support);
            }
        }

        Some(intersection)
    }
}

