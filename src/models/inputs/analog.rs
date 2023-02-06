use crate::models::has_state::HasState;
use crate::models::inputs::state::State;

struct Analog
{
    id: u16,
    pin: u32,
}

impl HasState for Analog
{
    fn read_state(&self) -> State {
        todo!()
    }
}