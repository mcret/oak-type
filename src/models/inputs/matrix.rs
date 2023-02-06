use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::slice::Iter;
use crate::models::has_state::HasState;
use crate::models::inputs;
use crate::models::inputs::state;
use crate::utils::Grouping;

struct Matrix
{
    id: String,
    elements: BTreeMap<u8, Vec<MatrixElement>>,
}

impl Matrix
{
    fn gather(&self) -> BTreeMap<u8, Vec<MatrixElement>>
    {
        let mut foo: Vec<MatrixElement> = Vec::new();
        foo.group_from(|element| element.location.0)
    }
}

impl HasState for Matrix {
    fn read_state(&self) -> state::State {
        todo!()
    }
}

struct MatrixElement
{
    id: String,
    location: (u8, u8),
}

impl HasState for MatrixElement {
    fn read_state(&self) -> state::State {
        todo!()
    }
}