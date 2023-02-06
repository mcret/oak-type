use crate::models::inputs::state::State;

pub trait HasState
{
    fn read_state(&self) -> State;
}