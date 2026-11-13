use crate::keys::Keys;
use crate::run::{Run, Values};
use crate::{Entry, Limit, Outcome};

/// Recent entries for each key, each key's read newest first.
#[derive(Clone, Debug)]
pub struct Groups<V> {
    size: usize,
    keys: Keys<Run<V>>,
}

impl<V> Groups<V> {
    /// An empty store holding at most `keys` keys, and for each what `values` allows.
    pub const fn new(keys: usize, values: Limit) -> Groups<V> {
        Groups {
            size: values.size,
            keys: Keys::new(keys, values.span),
        }
    }

    /// Gives the store an entry for `key`.
    pub fn insert(&mut self, time: i64, key: &str, value: V) -> Outcome {
        let Some(cutoff) = self.keys.see(time) else {
            return Outcome::Expired;
        };
        let size = self.size;
        let Some(run) = self.keys.get_mut(key) else {
            return self.add(time, key, value);
        };
        run.trim(cutoff);
        let was = run.newest().map(|newest| newest.time);
        let outcome = run.place(size, Entry { time, value });
        if let Some(was) = was.filter(|was| outcome.held() && time > *was) {
            self.keys.heard(key, was, time);
        }
        outcome
    }

    fn add(&mut self, time: i64, key: &str, value: V) -> Outcome {
        if self.size == 0 || !self.keys.admit(time, key) {
            return Outcome::Full;
        }
        let mut run = Run::new();
        run.place(self.size, Entry { time, value });
        self.keys.add(time, key, run);
        Outcome::Held
    }

    /// Tells the store of a time it takes no entry for.
    pub fn observe(&mut self, time: i64) {
        self.keys.see(time);
    }

    /// A key's entries, newest first.
    pub fn get(&self, key: &str) -> Option<Values<'_, V>> {
        let cutoff = self.keys.cutoff();
        self.keys.get(key).map(|run| run.live(cutoff))
    }

    /// The newest entry and its key.
    pub fn newest(&self) -> Option<(&str, &Entry<V>)> {
        let (key, run) = self.keys.newest()?;
        Some((key, run.newest()?))
    }

    /// Each key and its entries, in key order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, Values<'_, V>)> {
        let cutoff = self.keys.cutoff();
        self.keys
            .iter()
            .map(move |(key, run)| (key, run.live(cutoff)))
    }

    /// How many keys the store holds.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
