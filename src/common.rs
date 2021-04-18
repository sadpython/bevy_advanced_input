use std::hash::Hash;

use bevy::utils::HashMap;

pub trait InsertOrGet<K: Eq + Hash, V: Default> {
    fn insert_or_get(&mut self, item: K) -> &mut V;
}

impl<K: Eq + Hash, V: Default> InsertOrGet<K, V> for HashMap<K, V> {
    fn insert_or_get(&mut self, item: K) -> &mut V {
        return match self.entry(item) {
            std::collections::hash_map::Entry::Occupied(o) => o.into_mut(),
            std::collections::hash_map::Entry::Vacant(v) => v.insert(V::default()),
        };
    }
}