use std::io::{self, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> io::Result<()> {
    // Set terminal to raw mode to read key events
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;

    // Enable mouse mode to allow clicking events
    // Old code: separate write statements
    // write!(stdout, "{}", termion::cursor::Hide)?;
    // write!(stdout, "{}", termion::terminal::EnterMouseMode)?;

    // New code: combined write statement, reduces number of system calls
    write!(stdout, "{}{}", termion::cursor::Hide, termion::terminal::EnterMouseMode)?;

    // Listen for key events and print them to console
    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Char(c)) => println!("Key pressed: {}", c),
            Event::Key(Key::Esc) => break,
            _ => {}
        }
    }

    // Restore terminal state
    write!(stdout, "{}{}", termion::cursor::Show, termion::terminal::LeaveMouseMode)?;

    Ok(())
}
