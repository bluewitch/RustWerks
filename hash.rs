use std::env;
use std::fmt;

fn main() {
    // Equivalent to ARGV[0]
    let to_hash = env::args().nth(1).expect("Please provide a string to hash");
    // Compute the hash value
    let hash_value: u8 = to_hash.chars().map(|x| x as u8).sum::<u8>() % 256;
    // Convert to zero-padded hex
    let zero_padded_hash = format!("{:02X}", hash_value);
    // Output
    println!("Hash of '{}' is: {}", to_hash, zero_padded_hash);
}
