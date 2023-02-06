use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;
use core::slice::Iter;

pub trait Grouping: Iterator
{
    ///Groups elements of an Iterable into a map
    fn group_from<'a,S>(&self, f: fn(Self::Item) -> S) -> BTreeMap<S, Vec<Self::Item>>
    {
        let mut result: BTreeMap<S, Vec<Self::Item>> = BTreeMap::new();
        for thing in self {
            if let Some(x) = result.get_mut(f(&thing))
            {
                *x.push(thing)
            }
            else
            {
                let mut vector: Vec<Self::Item> = Vec::new();
                vector.push(thing);
                result.insert(f(&thing), vector)
            }
        };
        result
    }
}