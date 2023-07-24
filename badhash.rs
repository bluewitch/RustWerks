use std::collections::HashMap;
use std::hash::{Hasher, BuildHasherDefault};
use std::collections::hash_map::RandomState;

struct BadHash;

impl std::hash::Hasher for BadHash {
    fn write(&mut self, _bytes: &[u8]) {
        // Your implementation here
    }

    fn finish(&self) -> u64 {
        // Your implementation here
        42 // Dummy value
    }
}

type BadBuildHasher = BuildHasherDefault<BadHash>;

fn main() {
    let bad_hash = BadBuildHasher::default();
    let mut map = HashMap::with_hasher(bad_hash);

    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
}
