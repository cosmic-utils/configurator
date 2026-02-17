use indexmap::IndexMap;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug)]
pub struct Map<K, V>(pub IndexMap<K, V>);

/// Note: equality is only given if both values and order of values match
impl<K: Ord, V: Ord> PartialEq for Map<K, V> {
    fn eq(&self, other: &Map<K, V>) -> bool {
        self.cmp(other).is_eq()
    }
}

/// Note: equality is only given if both values and order of values match
impl<K: Ord, V: Ord> Eq for Map<K, V> {}

impl<K: Ord, V: Ord> PartialOrd for Map<K, V> {
    fn partial_cmp(&self, other: &Map<K, V>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V: Ord> Ord for Map<K, V> {
    fn cmp(&self, other: &Map<K, V>) -> Ordering {
        self.0.iter().cmp(other.0.iter())
    }
}

impl<K: Hash, V: Hash> Hash for Map<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().for_each(|x| x.hash(state));
    }
}
