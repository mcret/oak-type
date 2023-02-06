use alloc::vec::Vec;

pub enum State
{
    On,
    Off,
    Value(u16),
    Many(Vec<State>),
}