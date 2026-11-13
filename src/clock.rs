//! The newest time a store has seen, and how far back its `span` reaches from it.

#[derive(Clone, Copy, Debug)]
pub(crate) struct Clock {
    newest: Option<i64>,
    span: Option<u64>,
}

impl Clock {
    pub(crate) const fn new(span: Option<u64>) -> Clock {
        Clock { newest: None, span }
    }

    /// Takes a time, and returns the cutoff after it.
    pub(crate) fn see(&mut self, time: i64) -> i64 {
        self.newest = Some(self.newest.map_or(time, |newest| newest.max(time)));
        self.cutoff()
    }

    /// The oldest time `span` allows; every time, before the store has seen one or with no `span`.
    pub(crate) fn cutoff(&self) -> i64 {
        match (self.newest, self.span) {
            (Some(newest), Some(span)) => newest.saturating_sub_unsigned(span),
            _ => i64::MIN,
        }
    }
}
