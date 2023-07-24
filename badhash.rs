fn main() {
    let bad_hash = BadHash;
    let mut map = std::collections::HashMap::with_hasher(bad_hash);

    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
}
