// Import only what's needed
use std::hash::{BuildHasherDefault, Hasher, BuildHasher};
use std::collections::HashMap;

// Implement the Hasher trait for BadHash
struct BadHash;

impl Hasher for BadHash {
    fn finish(&self) -> u64 {
        0 // Return a constant for simplicity
    }

    fn write(&mut self, _bytes: &[u8]) {
        // Do nothing for simplicity
    }
}

// Implement the BuildHasher trait for BadHash
impl BuildHasher for BuildHasherDefault<BadHash> {
    type Hasher = BadHash;

    fn build_hasher(&self) -> Self::Hasher {
        BadHash
    }
}

// Now you can use BadHash as the hasher in your HashMap
fn main() {
    let mut map = HashMap::default(BuildHasherDefault::<BadHash>::default());
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
}
