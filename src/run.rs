//! Entries in time order, no two with one time.

use crate::{Entry, Outcome};
use std::collections::vec_deque::{Iter, VecDeque};
use std::iter::Rev;

#[derive(Clone, Debug)]
pub(crate) struct Run<V> {
    /// Oldest first.
    entries: VecDeque<Entry<V>>,
}

/// A list's entries, newest first.
#[derive(Clone, Debug)]
pub struct Values<'a, V>(Rev<Iter<'a, Entry<V>>>);

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a Entry<V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<V> ExactSizeIterator for Values<'_, V> {}

impl<V> Run<V> {
    pub(crate) const fn new() -> Run<V> {
        Run {
            entries: VecDeque::new(),
        }
    }

    /// Drops the entries older than `cutoff`.
    pub(crate) fn trim(&mut self, cutoff: i64) {
        while self.entries.front().is_some_and(|e| e.time < cutoff) {
            self.entries.pop_front();
        }
    }

    /// Puts an entry where its time places it. A full run drops its oldest, which may be the
    /// entry given.
    pub(crate) fn place(&mut self, size: usize, entry: Entry<V>) -> Outcome {
        let at = match self.entries.binary_search_by_key(&entry.time, |e| e.time) {
            Ok(_) => return Outcome::Duplicate,
            Err(at) => at,
        };
        if self.entries.len() < size {
            self.entries.insert(at, entry);
            return Outcome::Held;
        }
        if at == 0 {
            return Outcome::Full;
        }
        self.entries.pop_front();
        self.entries.insert(at.saturating_sub(1), entry);
        Outcome::Held
    }

    pub(crate) fn newest(&self) -> Option<&Entry<V>> {
        self.entries.back()
    }

    /// The entries no older than `cutoff`, newest first.
    pub(crate) fn live(&self, cutoff: i64) -> Values<'_, V> {
        let from = self.entries.partition_point(|e| e.time < cutoff);
        Values(self.entries.range(from..).rev())
    }
}
