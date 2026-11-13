use crate::keys::Keys;
use crate::{Entry, Limit, Outcome};
use std::cmp::Ordering;

/// The newest entry for each key.
#[derive(Clone, Debug)]
pub struct Map<V> {
    keys: Keys<Entry<V>>,
}

impl<V> Map<V> {
    /// An empty map holding the keys `keys` allows.
    pub const fn new(keys: Limit) -> Map<V> {
        Map {
            keys: Keys::new(keys.size, keys.span),
        }
    }

    /// Gives the map an entry for `key`.
    pub fn insert(&mut self, time: i64, key: &str, value: V) -> Outcome {
        if self.keys.see(time).is_none() {
            return Outcome::Expired;
        }
        let Some(held) = self.keys.get_mut(key) else {
            return self.add(time, key, value);
        };
        let was = held.time;
        match time.cmp(&was) {
            Ordering::Equal => Outcome::Duplicate,
            Ordering::Less => Outcome::Older,
            Ordering::Greater => {
                *held = Entry { time, value };
                self.keys.heard(key, was, time);
                Outcome::Held
            }
        }
    }

    fn add(&mut self, time: i64, key: &str, value: V) -> Outcome {
        if !self.keys.admit(time, key) {
            return Outcome::Full;
        }
        self.keys.add(time, key, Entry { time, value });
        Outcome::Held
    }

    /// Tells the map of a time it takes no entry for.
    pub fn observe(&mut self, time: i64) {
        self.keys.see(time);
    }

    pub fn get(&self, key: &str) -> Option<&Entry<V>> {
        self.keys.get(key)
    }

    /// The newest entry and its key.
    pub fn newest(&self) -> Option<(&str, &Entry<V>)> {
        self.keys.newest()
    }

    /// Each key and its entry, in key order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Entry<V>)> {
        self.keys.iter()
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
