use state_store::{Groups, Limit, Outcome};

fn times(groups: &Groups<u8>, key: &str) -> Vec<i64> {
    groups
        .get(key)
        .into_iter()
        .flatten()
        .map(|entry| entry.time)
        .collect()
}

fn keys(groups: &Groups<u8>) -> Vec<&str> {
    groups.iter().map(|(key, _)| key).collect()
}

#[test]
fn each_key_keeps_its_own_list() {
    let mut groups = Groups::new(
        4,
        Limit {
            size: 2,
            span: None,
        },
    );
    groups.insert(10, "a", 1);
    groups.insert(10, "b", 2);
    groups.insert(30, "a", 3);
    assert_eq!(groups.insert(30, "a", 5), Outcome::Duplicate);
    assert_eq!(groups.insert(20, "a", 4), Outcome::Held);
    assert_eq!(groups.insert(5, "a", 6), Outcome::Full);
    assert_eq!(times(&groups, "a"), [30, 20]);
    assert_eq!(times(&groups, "b"), [10]);
    assert!(groups.get("c").is_none());
    assert_eq!(
        groups.newest().map(|(key, entry)| (key, entry.time)),
        Some(("a", 30))
    );
}

#[test]
fn a_full_store_drops_the_least_recent_key_with_its_entries() {
    let mut groups = Groups::new(
        2,
        Limit {
            size: 3,
            span: None,
        },
    );
    groups.insert(10, "a", 1);
    groups.insert(20, "b", 2);
    groups.insert(30, "a", 3);
    assert_eq!(groups.insert(25, "c", 4), Outcome::Held);
    assert_eq!(keys(&groups), ["a", "c"]);
    assert_eq!(groups.insert(15, "d", 5), Outcome::Full);
    assert_eq!(times(&groups, "a"), [30, 10]);
}

#[test]
fn an_older_entry_leaves_its_key_as_recent_as_it_was() {
    let mut groups = Groups::new(
        2,
        Limit {
            size: 3,
            span: None,
        },
    );
    groups.insert(10, "a", 1);
    groups.insert(20, "b", 2);
    groups.insert(5, "b", 3);
    groups.insert(30, "c", 4);
    assert_eq!(keys(&groups), ["b", "c"]);
}

#[test]
fn span_drops_entries_and_then_empty_keys() {
    let mut groups = Groups::new(
        4,
        Limit {
            size: 3,
            span: Some(10),
        },
    );
    groups.insert(10, "a", 1);
    groups.insert(18, "a", 2);
    groups.insert(12, "b", 3);
    groups.insert(25, "c", 4);
    assert_eq!(times(&groups, "a"), [18]);
    assert_eq!(keys(&groups), ["a", "c"]);
    assert_eq!(groups.insert(14, "a", 5), Outcome::Expired);
    groups.observe(40);
    assert!(groups.is_empty());
}

#[test]
fn an_expired_entry_holds_no_place() {
    let mut groups = Groups::new(
        4,
        Limit {
            size: 2,
            span: Some(10),
        },
    );
    groups.insert(10, "a", 1);
    groups.insert(20, "a", 2);
    groups.insert(25, "b", 3);
    assert_eq!(groups.insert(16, "a", 4), Outcome::Held);
    assert_eq!(times(&groups, "a"), [20, 16]);
}
