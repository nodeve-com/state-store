use state_store::{Limit, List, Outcome};

fn times(list: &List<&str>) -> Vec<i64> {
    list.values().map(|entry| entry.time).collect()
}

#[test]
fn reads_newest_first_whatever_the_order_given() {
    let mut list = List::new(Limit {
        size: 4,
        span: None,
    });
    for time in [20, 40, 10, 30] {
        assert_eq!(list.insert(time, "v"), Outcome::Held);
    }
    assert_eq!(times(&list), [40, 30, 20, 10]);
    assert_eq!(list.newest().map(|entry| entry.time), Some(40));
}

#[test]
fn a_full_list_drops_its_oldest() {
    let mut list = List::new(Limit {
        size: 2,
        span: None,
    });
    list.insert(10, "a");
    list.insert(20, "b");
    assert_eq!(list.insert(15, "c"), Outcome::Held);
    assert_eq!(times(&list), [20, 15]);
    assert_eq!(list.insert(5, "d"), Outcome::Full);
    assert_eq!(times(&list), [20, 15]);
}

#[test]
fn span_drops_before_size_counts() {
    let mut list = List::new(Limit {
        size: 2,
        span: Some(10),
    });
    list.insert(10, "a");
    list.insert(20, "b");
    assert_eq!(list.insert(25, "c"), Outcome::Held);
    assert_eq!(times(&list), [25, 20]);
    assert_eq!(list.insert(14, "late"), Outcome::Expired);
    assert_eq!(list.insert(15, "edge"), Outcome::Full);
    assert_eq!(times(&list), [25, 20]);
}

#[test]
fn a_held_time_is_refused() {
    let mut list = List::new(Limit {
        size: 3,
        span: None,
    });
    list.insert(10, "first");
    assert_eq!(list.insert(10, "again"), Outcome::Duplicate);
    assert_eq!(
        list.values().map(|entry| entry.value).collect::<Vec<_>>(),
        ["first"]
    );
}

#[test]
fn observe_moves_time_on() {
    let mut list = List::new(Limit {
        size: 3,
        span: Some(10),
    });
    list.insert(10, "a");
    list.insert(18, "b");
    list.observe(25);
    assert_eq!(times(&list), [18]);
    list.observe(100);
    assert!(list.is_empty());
}

#[test]
fn a_list_of_no_size_holds_nothing() {
    let mut list = List::new(Limit {
        size: 0,
        span: None,
    });
    assert_eq!(list.insert(10, "a"), Outcome::Full);
    assert_eq!(list.len(), 0);
}
