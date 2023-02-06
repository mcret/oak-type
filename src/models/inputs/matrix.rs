use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use crate::models::has_state::HasState;
use crate::models::inputs;
use crate::utils;

struct Matrix
{
    id: String,
    positive_pins: Vec<u8>,
    read_pins: Vec<u8>,
    elements: Vec<MatrixElement>,
}

impl Matrix
{
    fn gather(&self) -> BTreeMap<u8, Vec<MatrixElement>>
    {
        self.elements
            .iter()
            .group_from()
    }
}

impl HasState for Matrix {
    fn read_state(&self) -> inputs::State {
        let states : Vec<inputs::State> = Vec::new();
        for positive_pin in &self.positive_pins {
            //todo: bring pin high
            for read_pin in &self.read_pins {
                &self.elements.get
            }
            //todo: bring pin low
        }
    }
}

struct MatrixElement
{
    id: String,
    location: (u8, u8),
}

impl HasState for MatrixElement {
    fn read_state(&self) -> inputs::State {
        todo!()
    }
}