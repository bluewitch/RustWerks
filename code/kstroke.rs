use std::io::{self, Read};
use termion::event::{Event, Key};
use termion::input::TermRead;

fn main() -> io::Result<()> {
    // Set terminal to raw mode to read key events
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;

    // Enable mouse mode to allow clicking events
    write!(stdout, "{}", termion::cursor::Hide)?;
    write!(stdout, "{}", termion::terminal::EnterMouseMode)?;

    // Listen for key events and print them to console
    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Char(c)) => println!("Key pressed: {}", c),
            Event::Key(Key::Esc) => break,
            _ => {}
        }
    }

    // Restore terminal state
    write!(stdout, "{}", termion::cursor::Show)?;
    write!(stdout, "{}", termion::terminal::LeaveMouseMode)?;
    Ok(())
}

// This code uses the termion crate to capture keyboard events and print them to the console. 
// The program listens for key events and prints the character associated with the key when it is pressed. 
// The program exits when the "Esc" key is pressed.
