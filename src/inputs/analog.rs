use alloc::string::String;
use crate::inputs::InputConfig;

struct Analog
{
    id: String,
    pin: u16,
}

impl InputConfig<u16> for Analog {}