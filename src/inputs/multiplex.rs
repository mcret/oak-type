use alloc::string::String;
use alloc::vec::Vec;

struct Multiplex
{
    id: String,
    control_pins: Vec<u8>,
    read_pin: u8,

}