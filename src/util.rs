use sdl2::rect::Rect as SDLRect;
use nalgebra::Vector2;
use crate::wchar_t;

pub type Vec2 = Vector2<f32>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rect {
    top_left: Vec2,
    bottom_right: Vec2
}
impl Rect {
    pub fn new(top_left: Vec2, bottom_right: Vec2) -> Self {
        Self { top_left, bottom_right }
    }

    pub fn from_size(top_left: Vec2, width: f32, height: f32) -> Self {
        Self {
            top_left,
            bottom_right: top_left + Vec2::new(width, height),
        }
    }

    pub fn top(&self) -> f32 { self.top_left.y }
    pub fn left(&self) -> f32 { self.top_left.x }
    pub fn right(&self) -> f32 { self.bottom_right.x }
    pub fn bottom(&self) -> f32 { self.bottom_right.y }

    pub fn top_left(&self) -> Vec2 { self.top_left }
    pub fn bottom_right(&self) -> Vec2 { self.bottom_right }
    pub fn top_right(&self) -> Vec2 { Vec2::new(self.bottom_right.x, self.top_left.y) }
    pub fn bottom_left(&self) -> Vec2 { Vec2::new(self.top_left.x, self.bottom_right.y) }

    pub fn width(&self) -> f32 { self.top_left.x - self.bottom_right.x }
    pub fn height(&self) -> f32 { self.top_left.y - self.bottom_right.y }

    pub fn verticies(&self) -> [Vec2; 4] {
        [
            self.top_left(),
            self.top_right(),
            self.bottom_left(),
            self.bottom_right(),
        ]
    }

    pub fn set_top_left(&mut self, top_left: Vec2) { self.top_left = top_left }
    pub fn set_bottom_right(&mut self, bottom_right: Vec2) { self.bottom_right = bottom_right }


    pub fn shifted(&self, translation: Vec2) -> Self {
        Self {
            top_left: self.top_left + translation,
            bottom_right: self.bottom_right + translation,
        }
    }

    pub fn enlarged(&self, scale: Vec2) -> Self {
        let size = (self.bottom_right - self.top_left).xy();
        Self {
            top_left: self.top_left - Vec2::new(size.x * scale.x / 2.0, size.y * scale.y / 2.0),
            bottom_right: self.bottom_right + Vec2::new(size.x * scale.x / 2.0, size.y * scale.y / 2.0),
        }
    }

    pub fn intersecting(&self, other: &Rect) -> bool {
        let a = self;
        let b = other;
        a.top_left().x < b.bottom_right().x && a.bottom_right().x > b.top_left().x
            && a.top_left().y < b.bottom_right().y && a.bottom_right().y > b.top_left().y
    }

    pub fn into_sdl2_rect(self) -> SDLRect {
        let size = (self.bottom_right - self.top_left).xy();
        SDLRect::new(
            self.top_left.x as i32,
            self.top_left.y as i32,
            size.x as u32,
            size.y as u32,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Polygon {
    vertices: Vec<Vec2>,
}
impl Polygon {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        Self { vertices }
    }

    pub fn support(&self, direction: Vec2) -> Vec2 {
        let mut furthest_distance = f32::NEG_INFINITY;
        let mut furthest_vertex = Vec2::default();

        for v in &self.vertices {
            let distance = v.dot(&direction);

            if distance > furthest_distance {
                furthest_distance = distance;
                furthest_vertex = *v;
            }
        }

        furthest_vertex
    }
}

