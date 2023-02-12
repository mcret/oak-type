use alloc::string::String;
use alloc::vec::Vec;
use crate::inputs::status::{HasStatus, Status};

struct Multiplex
{
    id: String,
    control_pins: Vec<u8>,
    read_pin: u8,

}

impl HasStatus for Multiplex {
    fn get_status(&self) -> Status {
        todo!()

    }
}