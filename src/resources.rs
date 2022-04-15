use std::collections::HashSet;
use std::hash::Hash;
use sdl2::keyboard::Keycode;
use sdl2::video::FullscreenType::Desktop;
use sdl2::rect::Rect as SDLRect;
use crate::util::{Rect, Vec2};

#[derive(Clone, Copy, Debug)]
pub enum SystemState {
    Running,
    Quit,
}
impl Default for SystemState {
    fn default() -> Self {
        Self::Running
    }
}

#[derive(Debug, Default)]
pub struct GameState {
    pub system_state: SystemState,
    pub keys_pressed: HashSet<Keycode>,
    pub keys_released: HashSet<Keycode>,
    pub keys_held: HashSet<Keycode>,
    pub delta_t: f32,
}
impl GameState {
    pub fn new(system_state: SystemState) -> Self {
        Self {
            system_state,
            keys_pressed: Default::default(),
            keys_released: Default::default(),
            keys_held: Default::default(),
            delta_t: 0.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct GameCamera {
    size: (u32, u32),
    scale: (f32, f32),
    pos: Vec2,
    screen: Rect,
}
impl GameCamera {
    pub fn new(size: (u32, u32), pos: Vec2, screen: Rect) -> Self {
        Self {
            size,
            scale: (size.0 as f32 / (screen.bottom_right().x - screen.top_left().x),
                    size.1 as f32 / (screen.bottom_right().y - screen.top_left().y)),
            pos,
            screen,
        }
    }

    pub fn get_size(&self) -> (u32, u32) { self.size }
    pub fn get_pos(&self) -> Vec2 { self.pos }
    pub fn get_screen(&self) -> Rect { self.screen }

    pub fn get_screen_point(&self, point: Vec2) -> (i32, i32) {
        let relative_pos = point - self.screen.top_left() - self.pos;
        ((relative_pos.x * self.scale.0) as i32, (relative_pos.y * self.scale.1) as i32)
    }
    pub fn try_get_screen_point(&self, point: Vec2) -> Option<(i32, i32)> {
        let relative_pos = point - self.screen.top_left() - self.pos;
        /*let outside = relative_pos.y > self.screen.top() || relative_pos.y < self.screen.bottom()
            || relative_pos.x > self.screen.right() || relative_pos.x < self.screen.left();*/

        Some(((relative_pos.x * self.scale.0) as i32, (relative_pos.y * self.scale.1) as i32))
    }
    pub fn process_rect(&self, pos: Vec2, rect: Rect) -> SDLRect {
        let pos = self.get_screen_point(pos);


        SDLRect::new(pos.0, pos.1, (rect.width() * self.scale.0) as u32, (rect.height() * self.scale.1) as u32)
    }
    pub fn try_process_rect(&self, pos: Vec2, rect: Rect) -> Option<SDLRect> {
        let pos = self.try_get_screen_point(rect.top_left() + pos)?;

        Some(SDLRect::new(
            pos.0 as i32,
            pos.1 as i32,
            (rect.width() * self.scale.0.abs()) as u32,
            (rect.height() * self.scale.1.abs()) as u32,
        ))
    }
}