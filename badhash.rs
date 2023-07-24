use std::hash::{Hash, Hasher};

struct BadHash;

struct BadHasher {
    hash: u64,
}

impl Hasher for BadHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, _: &[u8]) {
        // Do nothing, our hash is constant
    }
}

impl Default for BadHasher {
    fn default() -> BadHasher {
        BadHasher { hash: 42 }  // Always hash to 42
    }
}

impl std::hash::BuildHasher for BadHash {
    type Hasher = BadHasher;

    fn build_hasher(&self) -> BadHasher {
        BadHasher::default()
    }
}

// usage
use std::collections::HashMap;

let bad_hash = BadHash;
let mut map = HashMap::with_hasher(bad_hash);

map.insert("key1", "value1");
map.insert("key2", "value2");
map.insert("key3", "value3");
