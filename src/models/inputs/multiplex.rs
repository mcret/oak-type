use alloc::string::String;
use alloc::vec::Vec;
use crate::models::has_state::HasState;
use crate::models::inputs::state::State;

struct Multiplex
{
    id: String,
    control_pins: Vec<u8>,
    read_pin: u8,

}

impl HasState for Multiplex {
    fn read_state(&self) -> State {
        todo!()

    }
}