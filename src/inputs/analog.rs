use crate::inputs::status::Status;
use crate::inputs::state::Status;

struct Analog
{
    id: u16,
    pin: u32,
}

impl Status for Analog
{
    fn get_status(&self) -> Status {
        todo!()
    }
}