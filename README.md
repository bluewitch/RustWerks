# RustWerks
<img src="/images/wasm-ferris.png" align="left" width="500px"/>
My book (dwulf's manifesto) on Rust, notes, details, and references.

RustWerks is a manifesto for Rust specific development and a complete deep dive into 
all things Rust.  Rust was bootstrapped from an compiler written in OCaml. 
The compiler is also not exclusively written in Rust 
with the actual code generation and low level optimizations happen through the LLVM library, 
which is written in C++.

There is also the mrustc (https://github.com/thepowersgang/mrustc) project that can be used for bootstrapping. 
It compiles/transpiles Rust into C that can be compiled using something like Clang or GCC.

Techically speaking, Rust is not "written in anything". It is a language, the syntax, the rules, operations, everything is defined as an abstract concept that eventually is meant to be transcribed to the machine code, that when fed to the computer, does stuff.

A compiler is just a program that is transcribing a source code written in some language to the mentioned machine code. Like every other piece of software, it can be written in any language. Just like a Fibonacci sequence generator, like Fizzbuzz, like anything that does stuff. Input source code, output machine code.

So at first, the source code written in Rust was compiled to the machine code with a compiler written in another language (OCaml apparently, but it can be done in any language, so let's tell that it was C). So you had a program written in C that took source code written in Rust and returned machine code that can be executed on a computer.

Then after testing that the thing works, somebody wrote the program that does the same thing that this old C program did, but this time they written it in Rust. Then they compiled it with the C program.

Now you have a compiled executable originally written in Rust that can compile new things. So we took the executable, compiled the source code and it turned out to work. Thus Rust "compiled itself".


I use Github's dev browser tool
and VSCode IDE and have found it a great tool for development and git commits

# - [Github Browser Dev Tool](https://github.dev/github/dev)
The Github Dev Tool is invoked by pointing your browser to a github on github.com,
and pressing a '.' or the period button on your keyboard.  This will give in browser VSCode IDE

# - [bors-ng](https://github.com/bors-ng/bors-ng)
Bors-NG implements a continuous-testing workflow where the main branch never breaks. It integrates GitHub pull requests with a tool like Travis CI that runs your tests.

## In Rust we Trust:
*~dwulf*

> I live in code,
> my logic is real.
> my own abode,
> its electrons I steal.

> The code is Rust,
> with a compiler that knows.
> to never trust,
> the sytnax that still grows.

> Complication, 
> is a name of a game.
> that computation,
> will reduce just the same.

> To clarify,
> we code what we trust.
> To simplify,
> we code it in Rust

## Sanity Check
---
Sanity testing is performed to ensure that the code changes that are made are working as properly. 
Sanity testing is a stoppage to check whether testing for the build can proceed or not. 
The focus of the team during sanity testing process is to validate the functionality of the application and not detailed testing.
A sanity test isn't limited in any way to the context of programming or software engineering. 
A sanity test is just a casual term to mean that you're testing/confirming/validating something that should follow very clear and simple logic.


## Rust Games URLs
- [Bevy Game Engine for Rust](https://bevyengine.org/)


## Resource URLs
- [Kerkour Bloom](https://kerkour.com/), Software development and security tips from the field.
- [Blackhat Rust Github](https://github.com/skerkour/black-hat-rust), Code for the book
- [Rust Cheats](https://cheats.rs/), Cheat Sheet for Rust
- [Crates Live](https://crates.live/rand/0.8.4)
- [Web3-Stack](https://github.com/open-web3-stack/open-runtime-module-library)

## Rust Projects
- [Cargo Lambda](https://github.com/cargo-lambda/cargo-lambda)
- [C64](https://github.com/onnokort/semu-c64), yes this is C, but will be Rust
- [BitTorrent client](https://github.com/gabrieldemian/vincenzo)
- [rdpFX, A simple file explorer that was born because I wanted to learn the Rust language.](https://github.com/RickyDane/rdpFX)
- [sudo-rs, A safety oriented and memory safe implementation of sudo and su written in Rust](https://github.com/memorysafety/sudo-rs)
- [Comprehensive Rust 🦀, a multi-day Rust course developed by the Android](https://github.com/google/comprehensive-rust)
- [ebpfguard, a library for managing Linux security policies.](https://github.com/deepfence/ebpfguard)
- [wasmsnark, A fast zkSnark proof and verifier and proof generator written in native Web Assembly.](https://github.com/iden3/wasmsnark)
- []()
- [hyper-fast, rust based very fast HTTP Web framework,much faster than actix and other frameworks](https://github.com/hyper-fast/hyper-fast)
- [axum, is a web application framework that focuses on ergonomics and modularity.](https://github.com/tokio-rs/axum)
- [salvo, an extremely simple and powerful Rust web backend framework](https://github.com/salvo-rs/salvo)
- 
---

# [XCM Format (Dr Wood, Parity)](https://github.com/paritytech/xcm-format)
