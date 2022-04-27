use nalgebra::Vector2;
use sdl2::rect::Rect as SDLRect;

pub type Vec2 = Vector2<f32>;

pub trait Shape2D {
    fn vertices(&self) -> Vec<Vec2>;

    fn project(&self, axis: &Vec2) -> (f32, f32) {
        let vertices = self.vertices();

        let mut min = axis.dot(&vertices[0]);
        let mut max = min;

        for vertex in &vertices {
            let p = axis.dot(vertex);

            if p < min {
                min = p;
            } else if p > max {
                max = p;
            }
        }

        (min, max)
    }

    fn shifted(&self, shift: &Vec2) -> Box<dyn Shape2D>;

    fn get_axes(&self) -> Vec<Vec2> {
        let vertices = self.vertices();

        let mut axes = Vec::with_capacity(vertices.len());

        for i in 0..vertices.len() {
            let a = vertices[i];
            let b = vertices[if i + 1 >= vertices.len() { 0 } else { i + 1 }];

            let ba = a - b;

            axes.push(Vec2::new(ba.y, -ba.x).normalize());
        }

        axes
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Rect {
    top_left: Vec2,
    bottom_right: Vec2,
}
impl Rect {
    pub fn new(top_left: Vec2, bottom_right: Vec2) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn from_size(top_left: Vec2, width: f32, height: f32) -> Self {
        Self {
            top_left,
            bottom_right: top_left + Vec2::new(width, -height),
        }
    }
    pub fn from_centre(centre: Vec2, width: f32, height: f32) -> Self {
        Self {
            top_left: centre + Vec2::new(-width, height) / 2.0,
            bottom_right: centre + Vec2::new(width, -height) / 2.0,
        }
    }

    pub fn top(&self) -> f32 {
        self.top_left.y
    }
    pub fn left(&self) -> f32 {
        self.top_left.x
    }
    pub fn right(&self) -> f32 {
        self.bottom_right.x
    }
    pub fn bottom(&self) -> f32 {
        self.bottom_right.y
    }

    pub fn top_left(&self) -> Vec2 {
        self.top_left
    }
    pub fn bottom_right(&self) -> Vec2 {
        self.bottom_right
    }
    pub fn top_right(&self) -> Vec2 {
        Vec2::new(self.bottom_right.x, self.top_left.y)
    }
    pub fn bottom_left(&self) -> Vec2 {
        Vec2::new(self.top_left.x, self.bottom_right.y)
    }

    pub fn width(&self) -> f32 {
        self.bottom_right.x - self.top_left.x
    }
    pub fn height(&self) -> f32 {
        self.top_left.y - self.bottom_right.y
    }

    pub fn set_top_left(&mut self, top_left: Vec2) {
        self.top_left = top_left
    }
    pub fn set_bottom_right(&mut self, bottom_right: Vec2) {
        self.bottom_right = bottom_right
    }

    pub fn enlarged(&self, scale: Vec2) -> Self {
        let size = (self.bottom_right - self.top_left).xy();
        Self {
            top_left: self.top_left - Vec2::new(size.x * scale.x / 2.0, size.y * scale.y / 2.0),
            bottom_right: self.bottom_right
                + Vec2::new(size.x * scale.x / 2.0, size.y * scale.y / 2.0),
        }
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
impl Shape2D for Rect {
    fn vertices(&self) -> Vec<Vec2> {
        vec![
            self.top_left(),
            self.bottom_left(),
            self.bottom_right(),
            self.top_right(),
        ]
    }

    fn shifted(&self, shift: &Vec2) -> Box<dyn Shape2D> {
        Box::new(Self {
            top_left: self.top_left + shift,
            bottom_right: self.bottom_right + shift,
        })
    }

    fn get_axes(&self) -> Vec<Vec2> {
        let a1 = self.top_right() - self.top_left();
        let a1 = Vec2::new(a1.y, -a1.x).normalize();

        let a2 = self.bottom_right() - self.top_right();
        let a2 = Vec2::new(a2.y, -a2.x).normalize();

        vec![a1, a2]
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
impl Shape2D for Polygon {
    fn vertices(&self) -> Vec<Vec2> {
        self.vertices.clone()
    }

    fn shifted(&self, shift: &Vec2) -> Box<dyn Shape2D> {
        Box::new(Self {
            vertices: self.vertices.iter().map(|v| v + shift).collect(),
        })
    }
}

pub struct Incrementor {
    value: usize,
}
impl Incrementor {
    pub fn new() -> Self {
        Self { value: 1 }
    }
}
impl Iterator for Incrementor {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.value;
        self.value += 1;
        Some(current)
    }
}
