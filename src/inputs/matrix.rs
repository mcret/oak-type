use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

struct Matrix
{
    id: String,
    elements: BTreeMap<u8, Vec<MatrixElement>>,
}

struct MatrixElement
{
    id: String,
    positive_pin: u8,
    read_pin: u8,
}