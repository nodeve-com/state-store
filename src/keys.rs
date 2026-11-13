//! The keys a keyed store holds, each with its slot, and the order they drop in.

use crate::clock::Clock;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub(crate) struct Keys<S> {
    clock: Clock,
    size: usize,
    slots: BTreeMap<Arc<str>, S>,
    /// Each key under its newest entry's time: the first is the next to drop.
    recent: BTreeSet<(i64, Arc<str>)>,
}

impl<S> Keys<S> {
    pub(crate) const fn new(size: usize, span: Option<u64>) -> Keys<S> {
        Keys {
            clock: Clock::new(span),
            size,
            slots: BTreeMap::new(),
            recent: BTreeSet::new(),
        }
    }

    /// Takes a time and drops each key whose newest entry is now too old. Returns the cutoff,
    /// or none when `time` is itself too old.
    pub(crate) fn see(&mut self, time: i64) -> Option<i64> {
        let cutoff = self.clock.see(time);
        self.expire(cutoff);
        (time >= cutoff).then_some(cutoff)
    }

    /// The oldest time `span` allows.
    pub(crate) fn cutoff(&self) -> i64 {
        self.clock.cutoff()
    }

    fn expire(&mut self, cutoff: i64) {
        while self.recent.first().is_some_and(|(time, _)| *time < cutoff) {
            self.drop_first();
        }
    }

    fn drop_first(&mut self) {
        if let Some((_, key)) = self.recent.pop_first() {
            self.slots.remove(&key);
        }
    }

    /// Makes room for a new key whose entry holds `time`, dropping the least recent key from a
    /// full set. False when the new key is itself the least recent.
    pub(crate) fn admit(&mut self, time: i64, key: &str) -> bool {
        if self.slots.len() < self.size {
            return true;
        }
        let Some((oldest, name)) = self.recent.first() else {
            return false;
        };
        if (time, key) < (*oldest, &**name) {
            return false;
        }
        self.drop_first();
        true
    }

    pub(crate) fn add(&mut self, time: i64, key: &str, slot: S) {
        let key: Arc<str> = Arc::from(key);
        self.recent.insert((time, Arc::clone(&key)));
        self.slots.insert(key, slot);
    }

    /// Moves a held key from the time `was` to `now`.
    pub(crate) fn heard(&mut self, key: &str, was: i64, now: i64) {
        let Some((key, _)) = self.slots.get_key_value(key) else {
            return;
        };
        self.recent.remove(&(was, Arc::clone(key)));
        self.recent.insert((now, Arc::clone(key)));
    }

    pub(crate) fn get(&self, key: &str) -> Option<&S> {
        self.slots.get(key)
    }

    pub(crate) fn get_mut(&mut self, key: &str) -> Option<&mut S> {
        self.slots.get_mut(key)
    }

    /// The key heard from most recently, and its slot.
    pub(crate) fn newest(&self) -> Option<(&str, &S)> {
        let (_, key) = self.recent.last()?;
        Some((key, self.slots.get(key)?))
    }

    /// Each key and its slot, in key order.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&str, &S)> {
        self.slots.iter().map(|(key, slot)| (&**key, slot))
    }

    pub(crate) fn len(&self) -> usize {
        self.slots.len()
    }
}
