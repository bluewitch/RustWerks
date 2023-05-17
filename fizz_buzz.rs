// This is the main function, where the program starts execution.
fn main() {
    // The 'for' loop will iterate over each integer from 1 to 100 inclusive.
    // The '..=' syntax represents an inclusive range.
    for i in 1..=100 {
        // The 'if' statement checks if the current number (i) is divisible by both 3 and 5 (i.e., 15).
        // This check is done first because in the FizzBuzz game, "FizzBuzz" replaces numbers divisible by both 3 and 5.
        if i % 15 == 0 {
            // If the number is divisible by 15, then it's divisible by both 3 and 5, so we print "fizz buzz".
            println!("fizz buzz");
        } else if i % 3 == 0 {
            // If the number is not divisible by 15 but is divisible by 3, we print "fizz".
            println!("fizz");
        } else if i % 5 == 0 {
            // If the number is not divisible by 15 or 3 but is divisible by 5, we print "buzz".
            println!("buzz");
        } else {
            // If the number is not divisible by 3 or 5, we just print the number itself.
            println!("{}", i);
        }
    }
}

