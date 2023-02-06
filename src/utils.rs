use alloc::collections::BTreeMap;
use alloc::vec::Vec;

trait Grouping: Iterator
{
    ///Groups elements of an Iterable into a map
    fn group_from<S>(&self, f: fn(Iterator::Item) -> &S) -> BTreeMap<&S, Vec<Iterator::Item>>
    {
        let mut result: BTreeMap<&S, Vec<Iterator::Item>> = BTreeMap::new();
        for thing in self {
            if let Some(x) = result.get_mut(f(&thing))
            {
                *x.push(thing)
            }
            else
            {
                let mut vector = Vec::new();
                vector.push(thing);
                result.insert(f(&thing), vector)
            }
        };
        result
    }
}