use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::slice::Iter;
use crate::models::has_state::HasState;
use crate::models::inputs;
use crate::models::inputs::state;
use crate::models::inputs::state::State;

struct Matrix
{
    id: String,
    elements: BTreeMap<u8, Vec<MatrixElement>>,
}

impl HasState for Matrix {
    fn read_state(&self) -> state::State {
        let mut results = Vec::new();
        for x in self.elements.iter() {
            todo!()
        }
        State::Many(results)
    }
}

struct MatrixElement
{
    id: String,
    positive_pin: u8,
    read_pin: u8,
}