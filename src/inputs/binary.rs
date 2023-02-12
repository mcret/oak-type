use crate::inputs::status::Status;
use crate::inputs::state::Status;

struct Binary
{
    id: u16,
    pin: u32,
}

impl Status for Binary
{
    fn get_status(&self) -> Status {
        todo!()
    }
}