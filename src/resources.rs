use std::collections::HashSet;
use std::hash::Hash;
use sdl2::keyboard::Keycode;
use sdl2::video::FullscreenType::Desktop;

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