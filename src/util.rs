use sdl2::rect::Rect as SDLRect;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rect {
    top_left: Vec2,
    bottom_right: Vec2
}
impl Rect {
    pub fn new(top_left: Vec2, bottom_right: Vec2) -> Self {
        Self { top_left, bottom_right }
    }

    pub fn top_left(&self) -> Vec2 { self.top_left }
    pub fn bottom_right(&self) -> Vec2 { self.bottom_right }
    pub fn width(&self) -> f32 { self.top_left.x - self.bottom_right.x }
    pub fn height(&self) -> f32 { self.top_left.y - self.bottom_right.y }

    pub fn set_top_left(&mut self, top_left: Vec2) { self.top_left = top_left }
    pub fn set_bottom_right(&mut self, bottom_right: Vec2) { self.bottom_right = bottom_right }

    pub fn intersecting(&self, other: &Rect) -> bool {
        self.x < other.x + other.width && self.x + self.width > other.x
            && self.y < other.y + other.height && self.y + self.height > other.y
    }

    pub fn shifted(&self, x: f32, y: f32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            width: self.width,
            height: self.height
        }
    }

    pub fn enlarged(&self, x_sf: f32, y_sf: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            width: self.width * x_sf,
            height: self.height * y_sf,
        }
    }

    pub fn into_sdl2_rect(&self) -> SDLRect {
        SDLRect::new(self.x as i32, self.y as i32, self.width as u32, self.height as u32)
    }
}