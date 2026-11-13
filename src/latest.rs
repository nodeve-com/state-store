use crate::clock::Clock;
use crate::{Entry, Outcome};
use std::cmp::Ordering;

/// The newest entry.
#[derive(Clone, Debug)]
pub struct Latest<V> {
    clock: Clock,
    held: Option<Entry<V>>,
}

impl<V> Latest<V> {
    /// An empty store whose entry drops once older than `span`.
    pub const fn new(span: Option<u64>) -> Latest<V> {
        Latest {
            clock: Clock::new(span),
            held: None,
        }
    }

    /// Gives the store an entry.
    pub fn insert(&mut self, time: i64, value: V) -> Outcome {
        let cutoff = self.clock.see(time);
        self.expire(cutoff);
        if time < cutoff {
            return Outcome::Expired;
        }
        match self.held.as_ref().map(|held| time.cmp(&held.time)) {
            Some(Ordering::Equal) => Outcome::Duplicate,
            Some(Ordering::Less) => Outcome::Older,
            Some(Ordering::Greater) | None => {
                self.held = Some(Entry { time, value });
                Outcome::Held
            }
        }
    }

    /// Tells the store of a time it takes no entry for.
    pub fn observe(&mut self, time: i64) {
        let cutoff = self.clock.see(time);
        self.expire(cutoff);
    }

    pub const fn get(&self) -> Option<&Entry<V>> {
        self.held.as_ref()
    }

    fn expire(&mut self, cutoff: i64) {
        if self.held.as_ref().is_some_and(|held| held.time < cutoff) {
            self.held = None;
        }
    }
}
