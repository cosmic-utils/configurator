use indexmap::IndexMap;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug)]
pub struct Map<K, V>(pub IndexMap<K, V>);

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        Self(IndexMap::new())
    }

    pub fn with_capacity(n: usize) -> Self {
        Self(IndexMap::with_capacity(n))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> indexmap::map::Iter<'_, K, V> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> indexmap::map::IterMut<'_, K, V> {
        self.0.iter_mut()
    }
}

impl<K: Hash + Eq, V> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Map(IndexMap::from_iter(iter))
    }
}

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
