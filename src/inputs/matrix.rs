use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::slice::Iter;
use crate::inputs::status::{HasStatus, Status};

struct Matrix
{
    id: String,
    elements: BTreeMap<u8, Vec<MatrixElement>>,
}

impl HasStatus for Matrix {
    fn get_status(&self) -> Status {
        let mut results = Vec::new();
        for x in self.elements.iter() {
            todo!()
        }
        Status::Many(results)
    }
}

struct MatrixElement
{
    id: String,
    positive_pin: u8,
    read_pin: u8,
}