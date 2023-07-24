use std::env;

fn main() {
    let to_hash = env::args().nth(1).expect("Please provide a string to hash");

    // Compute the hash value.
    // We can use `as u32` to safely handle Unicode characters that don't fit in u8.
    let hash_value: u32 = to_hash.chars().map(|x| x as u32).sum::<u32>() % 256;

    // Now we can safely cast hash_value to u8 as it will be in 0..=255 range.
    let zero_padded_hash = format!("{:02X}", hash_value as u8);

    println!("Hash of '{}' is: {}", to_hash, zero_padded_hash);
}

