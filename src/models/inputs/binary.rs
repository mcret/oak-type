use crate::models::has_state::HasState;
use crate::models::inputs::state::State;

struct Binary
{
    id: u16,
    pin: u32,
}

impl HasState for Binary
{
    fn read_state(&self) -> State {
        todo!()
    }
}