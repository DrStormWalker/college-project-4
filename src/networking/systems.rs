use specs::{AccessorCow, RunningTime, System};
use crate::World;

pub struct NetworkPortal;
impl<'a> System<'a> for TransmitionNetworkPortal {
    type SystemData = ();
    
    fn run(&mut self, data: Self::SystemData) {

    }
}