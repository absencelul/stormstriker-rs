pub mod actors;

use sdk::engine::classes::ULocalPlayer;
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::{LazyLock, Mutex};

pub struct GenericCache<T>
where
    T: Eq + Hash + Copy,
{
    old_items: HashSet<T>,
    items: HashSet<T>,
}

impl<T> GenericCache<T>
where
    T: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            old_items: HashSet::new(),
            items: HashSet::new(),
        }
    }

    pub fn update(&mut self, new_items: Vec<T>, is_valid: fn(T) -> bool) {
        let new_set: HashSet<T> = HashSet::from_iter(new_items);
        let added_items = new_set.difference(&self.old_items);
        let removed_items = self.old_items.difference(&new_set);

        for &item in added_items {
            if is_valid(item) {
                self.items.insert(item);
            }
        }

        for &item in removed_items {
            self.items.remove(&item);
        }

        self.old_items = new_set;
    }
}
