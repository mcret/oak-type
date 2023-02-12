use crate::inputs::status::{HasStatus, Status};

struct Binary
{
    id: u16,
    pin: u32,
}

impl HasStatus for Binary
{
    fn get_status(&self) -> Status {
        todo!()
    }
}