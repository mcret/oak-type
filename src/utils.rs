use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;

pub trait Grouping
{
    type Item;
    ///Groups elements of an Iterable into a map
    fn group_from(&self, f: fn(&Self::Item) -> u8) -> BTreeMap<u8, Vec<Self::Item>>;
}

impl<T> Grouping for Vec<T>
{
    type Item = T;

    fn group_from(&self, f: fn(&Self::Item) -> u8) -> BTreeMap<u8, Vec<Self::Item>>
    {
        let mut result: BTreeMap<u8, Vec<T>> = BTreeMap::new();
        for thing in self.iter() {
            if let Some(x) = result.get_mut(&f(thing))
            {
                x.push(thing);
            } else {
                let mut vector: Vec<Self::Item> = Vec::new();
                vector.push(thing);
                result.insert(f(thing), vector)
            }
        }

        result
    }
}