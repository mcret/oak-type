use alloc::string::String;
use alloc::vec::Vec;
use crate::inputs::status::Status;
use crate::inputs::state::Status;

struct Multiplex
{
    id: String,
    control_pins: Vec<u8>,
    read_pin: u8,

}

impl Status for Multiplex {
    fn get_status(&self) -> Status {
        todo!()

    }
}