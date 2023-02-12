use crate::inputs::status::{HasStatus, Status};

struct Analog
{
    id: u16,
    pin: u32,
}

impl HasStatus for Analog
{
    fn get_status(&self) -> Status {
        todo!()
    }
}