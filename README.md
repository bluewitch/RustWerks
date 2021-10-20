# RustWerks
My book (dwulf's manifesto) on Rust, notes, details, and references.

RustWerks is a manifesto for Rust specific development and a complete deep dive into 
all things Rust.  Rust was bootstrapped from an compiler written in OCaml. 
The compiler is also not exclusively written in Rust 
with the actual code generation and low level optimizations happen through the LLVM library, 
which is written in C++.

There is also the mrustc (https://github.com/thepowersgang/mrustc) project that can be used for bootstrapping. 
It compiles/transpiles Rust into C that can be compiled using something like Clang or GCC.


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

---

# [XCM Format (Dr Wood, Parity)](https://github.com/paritytech/xcm-format)
