# state-store

Recent entries kept in memory, bounded by count and by age.

An **entry** is a value and the time it carries. A **store** holds the entries a program gives it, drops the ones its rules refuse, and reads back what remains. A program sets the rules once, when it makes the store.

Use it where a program reads what arrived before now: the last reading, the last hour of readings, or the last reading from each device.

```rust
use state_store::{Limit, List, Outcome};

let mut readings = List::new(Limit { size: 3, span: Some(60) });
assert_eq!(readings.insert(100, 21.5), Outcome::Held);
assert_eq!(readings.insert(130, 22.0), Outcome::Held);
assert_eq!(readings.insert(30, 19.0), Outcome::Expired);

let newest_first: Vec<f64> = readings.values().map(|entry| entry.value).collect();
assert_eq!(newest_first, [22.0, 21.5]);
```

## Stores

Each store is generic over its value, the type `V` in `List<V>`, and never looks inside one.

| Store | Holds | Made with | Reads |
| - | - | - | - |
| `Latest` | the newest entry | a `span` | `get` |
| `List` | recent entries | a `Limit` on its entries | `values`, `newest`, `len` |
| `Map` | the newest entry for each key | a `Limit` on its keys | `get`, `newest`, `iter`, `len` |
| `Groups` | recent entries for each key | a count of keys, and a `Limit` on each key's entries | `get`, `newest`, `iter`, `len` |

A **key** is text naming what an entry belongs to, such as a device. A list reads newest first. `iter` reads keys in key order. `newest` on a keyed store gives the newest entry with its key.

## Rules

A `Limit` holds a `size` and an optional `span`.

- **`size`** caps a count: a list's entries, or a keyed store's keys.
- **`span`** caps an age, in whatever unit the times are in. It counts back from the newest time the store has seen. An entry older than that time less `span` drops; one exactly `span` old stays.

A store reads no clock. It learns the time from each entry a program gives it, and from `observe`, which tells it of a time without an entry. A program whose time moves on without new entries calls `observe` so old entries still drop.

Every entry goes through the same steps, in order, whether it arrives in time order or late:

1. **`span` first.** The store takes the entry's time, drops what is now too old, and refuses the entry itself when too old: `Expired`.
2. **One entry per time.** A list refuses an entry whose time one of its entries holds: `Duplicate`. A keyed store compares the times of one key, never those of two keys.
3. **Its place by time.** A list puts the entry between its older and newer entries. A `Latest`, or a `Map` key, takes it only when newer than the one held; otherwise `Older`.
4. **`size` last.** A full list drops its oldest entry to make room. A full keyed store drops the key whose newest entry is oldest, with all it holds. If the entry given would be the one dropped, the store refuses it: `Full`.

- **A key with no entries drops.** In a `Map` and in `Groups`, `span` drops a key once its newest entry is too old.
- **Two keys equally recent** drop lowest key first.
- **`insert` returns an `Outcome`**: `Held`, or which rule refused the entry.

## Limits of the crate

- It saves nothing: a store is empty when made, and gone when dropped.
- It depends on no other crate.
