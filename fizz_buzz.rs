fn main() {
    for i in 1..=100 {
        if i % 15 == 0 {
            println!("fizz buzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}
