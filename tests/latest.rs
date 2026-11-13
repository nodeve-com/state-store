use state_store::{Latest, Outcome};

#[test]
fn takes_only_a_newer_entry() {
    let mut latest = Latest::new(None);
    assert!(latest.get().is_none());
    assert_eq!(latest.insert(20, "b"), Outcome::Held);
    assert_eq!(latest.insert(10, "a"), Outcome::Older);
    assert_eq!(latest.insert(20, "again"), Outcome::Duplicate);
    assert_eq!(latest.get().map(|entry| entry.value), Some("b"));
    assert_eq!(latest.insert(30, "c"), Outcome::Held);
    assert_eq!(latest.get().map(|entry| entry.time), Some(30));
}

#[test]
fn an_entry_past_its_span_leaves_the_store_empty() {
    let mut latest = Latest::new(Some(10));
    latest.insert(20, "a");
    latest.observe(30);
    assert!(latest.get().is_some());
    latest.observe(31);
    assert!(latest.get().is_none());
    assert_eq!(latest.insert(20, "late"), Outcome::Expired);
    assert_eq!(latest.insert(25, "b"), Outcome::Held);
}
