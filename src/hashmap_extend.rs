use std::collections::HashMap;

fn extend_hash_maps() {
    let mut h1 = HashMap::from([("foo", 1), ("bar", 2), ("baz", 3)]);
    let h2 = HashMap::from([("baz", 1), ("qux", 2)]);
    h1.extend(h2);
}
