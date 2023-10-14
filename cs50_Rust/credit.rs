// Importing the standard IO library for user input.
use std::io;

// Function to prompt the user for input and read it.
fn get_input(prompt: &str) -> i64 {
    println!("{}", prompt);  // Display the prompt.
    
    let mut input = String::new();
    // Read the user's input into the 'input' string.
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim and parse the string into an i64 number. Panic if invalid.
    input.trim().parse().expect("Please enter a valid number")
}

// Function to extract two adjacent digits from a number given a divisor.
// Example: Given number=1234 and divisor=10, it extracts digits 3 and 4.
fn extract_digits(num: i64, divisor: i64) -> (i64, i64) {
    let digit1 = (num % (divisor * 10) - num % divisor) / divisor;
    let digit2 = (num % (divisor * 100) - num % (divisor * 10)) / (divisor * 10);
    (digit1, digit2)
}

fn main() {
    // Fetch the credit card number from the user.
    let cc = get_input("Number: ");
    
    // Variables to store the sum of manipulated digits.
    let mut fn_sum = 0;
    let mut other_sum = 0;

    // Loop to manipulate and sum every other digit.
    for i in (0..8).rev() {
        let divisor = 10_i64.pow(i as u32);
        let (odd_digit, even_digit) = extract_digits(cc, divisor);
        
        // Multiply the even-positioned digit by 2.
        let multiplied = even_digit * 2;
        
        // Split the resulting number into two digits and add to the sum.
        fn_sum += multiplied % 10 + multiplied / 10;
        
        // Add the odd-positioned digits.
        other_sum += odd_digit;
    }

    // Calculate the checksum.
    let checksum = (fn_sum + other_sum) % 10;

    // Use Rust's pattern matching to determine the type of card.
    match (checksum, cc) {
        // If checksum is 0 and length is 13, and starts with 4.
        (0, 1000000000000..=9999999999999) if cc / 1000000000000 == 4 => println!("VISA"),
        // If checksum is 0 and length is 15, and starts with 34 or 37.
        (0, 10000000000000..=99999999999999) if [34, 37].contains(&(cc / 10000000000000)) => println!("AMEX"),
        // If checksum is 0 and length is 16, and starts with 51 to 55.
        (0, 100000000000000..=999999999999999) if (51..=55).contains(&(cc / 100000000000000)) => println!("MASTERCARD"),
        // If checksum is 0 and length is 16, and starts with 4.
        (0, 1000000000000000..=9999999999999999) if cc / 1000000000000000 == 4 => println!("VISA"),
        // For all other cases, it's invalid.
        _ => println!("INVALID"),
    }
}
