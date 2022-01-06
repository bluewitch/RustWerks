# Rust Language


Rust is a multi-paradigm system programming language focused on programming safety, meaning preventing fault tolorant errors, segmentation faults, especially safe concurrency.

Rust has syntax is similar to C and C++, JavaScript, if you are comming from C/C++ programming language Rust is designed to provide better memory safety while maintaining high performance.  Rust helps prevent many of the amature mistakes one can make with a more unprotected core language like C and C++ built with built in safe guards for fault tolorant errors, segmentation faults, which are common in low level languages

Compiles to an executable binary

### LLVM backend
LLVM suite of optimizations

The LLVM Project is a collection of modular and reusable compiler and toolchain technologies. Despite its name, LLVM has little to do with traditional virtual machines. The name "LLVM" itself is not an acronym; it is the full name of the project.


### Safe by default

- Sophisticated type system and analysis

- No segmintation faults, (seg-faults) In computing, a segmentation fault or access violation is a fault, or failure condition, raised by hardware with memory protection, notifying an operating system the software has attempted to access a restricted area of memory.
- No buffer overflows
- No null pointers, In computing, a null pointer or null reference is a value saved for indicating that the pointer or reference does not refer to a valid object.
- No dangling pointers, Dangling pointers and wild pointers in computer programming are pointers that do not point to a valid object of the appropriate type.  Borrowing prevents dangling pointers.


### No data races
Eliminates Data Races, A data race occurs when: two or more threads in a single process access the same memory location concurrently, and. at least one of the accesses is for writing, and the threads are not using any exclusive locks to control their accesses to that memory.

In JavaScript, hoisting and bubbling, where actions are running depending where the code was scripted in the file containing the code.

- At least one is unsynchronized
- At least one is write

Concurrency Bugs
- Segfaults
- Use-after-free
- Double Free
- Deadlock, starvation, or spinning (All are exploitable)

Rust prevents this

- Ownership guarantees
- No garbage collection
- Safe strong abstractions

Servo
Parallel layout engine written in Rust.

Ownership
Only one
```
fn main() {
   let mut v = Vec::new();
   v.push(li);
   drop(v);

   v.push(3);
   // ~^ ERROR: use of moved value:  `v`

}
```


### Memory Management
Rust has fine-grained memory management, but is automatically managed once created.

The Memory Management, is actually the compiler enforcing proper code layout, to be mindful and accountable with system memory, by simply not compiling until any issues are

### Mutability
Values are immutable by default, and must be tagged as mutable, preceeded with the keyword `mut`


Saftey

Spawn
```
fn main() {
   spawn(proc() {
    println!("Hello, ");
});

println!("World!!");
}


//Channels
fn main() {
   let (tx, rx) = channel();
   let tx2 = tx.clone();
   spawn(proc() tx.send(5i));
   spawn(proc() tx2.send(4));

   // Print 4 and 5 in  an unspecified order

   println!("{}", rx.recv());
   println!("{}", rx.recv());
}

// Channel Select
fn main() {
   let (tx, rx) = channel();
   let (quit_tx, quit_rx) =
channel();

   select! {
   () = rx.recv => { /*...*/ },
   () = quit_rx.recv() => { /*...*/ }
}
}

ARC Pointer
Atomic Reference Count pointer

use std::sync::Arc;

fn main() {
  let shared_numbers = Arc::new(vec![li,2,3]);
   let child_numbers = shared_numbers.clone();

spawn(proc() {
   let local_numbers = child_numbers.as_slice();
   // ...
});
let local_numbers = shared_numbers.as_slice();
}
```



std::sync
Barriers
RWLock
Semaphores
Once


Atomic primitives
Non blocking queues
Concurrent Hashtables
Lightweight thread pools
Futures
CILK-style fork-join concurrency

```
unsafe {
   ...
}
```
Useful for
Uninitialized memory
Interfacing with C code
Building parallel abstractions like ARC

Ownership / Borrowing permit creating safe abstraction bounderies

 
