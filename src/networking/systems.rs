use specs::{AccessorCow, RunningTime, System};
use crate::World;

pub struct TransmissionNetworkPortal;
impl<'a> System<'a> for TransmissionNetworkPortal {
    type SystemData = ();
    
    fn run(&mut self, data: Self::SystemData) {

    }
}