extern crate nix;
use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::Pid;
use std::process::Command;
use std::ptr::null_mut;

fn main() {
    // Replace with the path to your target executable
    let target_executable = "your_target_executable";

    // Launch the target process
    let child = Command::new(target_executable)
        .spawn()
        .expect("Failed to start the target process");

    // Get the PID of the child process
    let child_pid = Pid::from_raw(child.id() as i32);

    // Attach to the child process
    ptrace::attach(child_pid).expect("Failed to attach to the child process");

    // Wait for the child process to stop
    match waitpid(child_pid, None) {
        Ok(WaitStatus::Stopped(_, _)) => {
            println!("Child process stopped");

            // Read memory from the child process (example: reading 8 bytes at address 0x1000)
            let addr = 0x1000 as usize;
            let mut data: i64 = 0;

            // Safety: null_mut() is safe because we'll write to it through ptrace
            let data_ptr = &mut data as *mut i64 as *mut std::ffi::c_void;

            ptrace::read(child_pid, data_ptr, addr, None)
                .expect("Failed to read memory");

            println!("Read data from memory: {:?}", data);

            // Detach from the child process
            ptrace::detach(child_pid, None).expect("Failed to detach from the child process");
        }
        _ => {
            println!("Child process not in a stopped state");
        }
    }
}
