use alloc::vec::Vec;

pub trait HasStatus
{
    fn get_status(&self) -> Status;
}

pub enum Status
{
    On,
    Off,
    Value(u16),
    Many(Vec<Status>),
}

struct BoardState
{
    state: Vec<Status>
}