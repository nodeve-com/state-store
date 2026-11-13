use state_store::{Limit, Map, Outcome};

fn keys(map: &Map<u8>) -> Vec<&str> {
    map.iter().map(|(key, _)| key).collect()
}

#[test]
fn holds_the_newest_entry_for_each_key() {
    let mut map = Map::new(Limit {
        size: 4,
        span: None,
    });
    assert_eq!(map.insert(10, "a", 1), Outcome::Held);
    assert_eq!(map.insert(20, "b", 2), Outcome::Held);
    assert_eq!(map.insert(30, "a", 3), Outcome::Held);
    assert_eq!(map.insert(25, "a", 4), Outcome::Older);
    assert_eq!(map.insert(20, "b", 5), Outcome::Duplicate);
    assert_eq!(map.get("a").map(|entry| entry.value), Some(3));
    assert_eq!(map.get("b").map(|entry| entry.value), Some(2));
    assert_eq!(
        map.newest().map(|(key, entry)| (key, entry.time)),
        Some(("a", 30))
    );
    assert_eq!(map.len(), 2);
}

#[test]
fn a_full_map_drops_the_least_recent_key() {
    let mut map = Map::new(Limit {
        size: 2,
        span: None,
    });
    map.insert(10, "a", 1);
    map.insert(20, "b", 2);
    map.insert(30, "a", 3);
    assert_eq!(map.insert(25, "c", 4), Outcome::Held);
    assert_eq!(keys(&map), ["a", "c"]);
    assert_eq!(map.insert(5, "d", 5), Outcome::Full);
    assert_eq!(keys(&map), ["a", "c"]);
}

#[test]
fn span_drops_a_key_whose_entry_is_too_old() {
    let mut map = Map::new(Limit {
        size: 4,
        span: Some(10),
    });
    map.insert(10, "a", 1);
    map.insert(15, "b", 2);
    map.insert(21, "c", 3);
    assert_eq!(keys(&map), ["b", "c"]);
    assert_eq!(map.insert(10, "a", 1), Outcome::Expired);
    map.observe(40);
    assert!(map.is_empty());
}

#[test]
fn keys_equally_recent_drop_lowest_first() {
    let mut map = Map::new(Limit {
        size: 2,
        span: None,
    });
    map.insert(10, "b", 1);
    map.insert(10, "a", 2);
    map.insert(10, "c", 3);
    assert_eq!(keys(&map), ["b", "c"]);
}
