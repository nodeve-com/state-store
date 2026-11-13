use crate::clock::Clock;
use crate::run::{Run, Values};
use crate::{Entry, Limit, Outcome};

/// Recent entries, read newest first.
#[derive(Clone, Debug)]
pub struct List<V> {
    clock: Clock,
    size: usize,
    run: Run<V>,
}

impl<V> List<V> {
    /// An empty list holding what `limit` allows.
    pub const fn new(limit: Limit) -> List<V> {
        List {
            clock: Clock::new(limit.span),
            size: limit.size,
            run: Run::new(),
        }
    }

    /// Gives the list an entry.
    pub fn insert(&mut self, time: i64, value: V) -> Outcome {
        let cutoff = self.clock.see(time);
        self.run.trim(cutoff);
        if time < cutoff {
            return Outcome::Expired;
        }
        self.run.place(self.size, Entry { time, value })
    }

    /// Tells the list of a time it takes no entry for.
    pub fn observe(&mut self, time: i64) {
        let cutoff = self.clock.see(time);
        self.run.trim(cutoff);
    }

    /// The entries, newest first.
    pub fn values(&self) -> Values<'_, V> {
        self.run.live(self.clock.cutoff())
    }

    pub fn newest(&self) -> Option<&Entry<V>> {
        self.run.newest()
    }

    pub fn len(&self) -> usize {
        self.values().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
