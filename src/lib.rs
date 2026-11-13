#![doc = include_str!("../README.md")]

mod clock;
mod groups;
mod keys;
mod latest;
mod list;
mod map;
mod run;

pub use groups::Groups;
pub use latest::Latest;
pub use list::List;
pub use map::Map;
pub use run::Values;

/// One held entry: the time it carries and its value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry<V> {
    pub time: i64,
    pub value: V,
}

/// What bounds a set of entries or of keys: at most `size`, none older than `span` behind the
/// newest time the store has seen. `span` is in the unit the times are in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limit {
    pub size: usize,
    pub span: Option<u64>,
}

/// What a store did with an entry it was given.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The store holds it.
    Held,
    /// It is older than `span` allows.
    Expired,
    /// The store holds an entry with its time, and its key in a keyed store.
    Duplicate,
    /// The store holds a newer entry in the one place it would take.
    Older,
    /// The store is full and it would be the first to drop.
    Full,
}

impl Outcome {
    /// Whether the store holds the entry.
    pub const fn held(self) -> bool {
        matches!(self, Outcome::Held)
    }
}
