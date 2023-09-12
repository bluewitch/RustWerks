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



## Resource URLs
- [Kerkour Bloom](https://kerkour.com/), Software development and security tips from the field.
- [Blackhat Rust Github](https://github.com/skerkour/black-hat-rust), Code for the book
- [Rust Cheats](https://cheats.rs/), Cheat Sheet for Rust
- [Crates Live](https://crates.live/rand/0.8.4)
- [Web3-Stack](https://github.com/open-web3-stack/open-runtime-module-library)

## Rust Projects
The following are Rust specific projects that I feel are good learning tools, I find that when I read code I can write it better.
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
- [riscv-crypto, RISC-V cryptography extensions standardisation work](https://github.com/riscv/riscv-crypto)
---


## Operating System:
- [Theseus, is a modern OS written from scratch in Rust that explores 𝐢𝐧𝐭𝐫𝐚𝐥𝐢𝐧𝐠𝐮𝐚𝐥 𝐝𝐞𝐬𝐢𝐠𝐧](https://github.com/theseus-os/Theseus)
- [Redox, is an operating system written in Rust](https://github.com/redox-os/redox)

## Game Engine:
- [Bevy Game Engine org site](https://bevyengine.org/)
- [Bevy, is a refreshingly simple data-driven game engine built in Rust](https://github.com/bevyengine/bevy)
- [Fyrox, 3D and 2D game engine written in Rust](https://github.com/FyroxEngine/Fyrox)

## Social Media:
- [Lemmy, A link aggregator and forum for the fediverse](https://github.com/LemmyNet/lemmy)
- [Plume, is a federated blogging engine](https://github.com/Plume-org/Plume)

## Media Player:
- [Glide, Linux/macOS media player based on GStreamer and GTK](https://github.com/philn/glide)

## Pastebin:
- [Microbin, Secure, configurable file-sharing and URL shortening web app written in Rust.](https://github.com/szabodanika/microbin)

## Code Forge:
- [Gitoxide, git based VCS, Idiomatic, lean, fast & safe pure Rust implementation of Git](https://github.com/Byron/gitoxide)
- [Pijul, (New VCS)](https://nest.pijul.com/pijul/pijul)

## File Encryption Tool:
- [Rage, A simple, modern, and secure file encryption tool, using the age format](https://github.com/str4d/rage)

## Signing Tool:
- [rust-minisign, A pure Rust implementation of the Minisign signature tool](https://github.com/jedisct1/rust-minisign)
- [Signify, Create cryptographic signatures for files and verify them](https://github.com/badboy/signify-rs)

## Static Site Generator:
- [Zola, A fast static site generator in a single binary with everything built-in](https://github.com/getzola/zola)
- [Cobalt, A static site generator written in Rust](https://github.com/cobalt-org/cobalt.rs)

## Markdown based Doc Generator:
- [mdBook, is a utility to create modern online books from Markdown files](https://github.com/rust-lang/mdBook)

## Frontend Web Framework w/ WASM Support:
- [Perseus, state-driven web development framework for Rust with full support for server-side rendering and static generation](https://github.com/framesurge/perseus)
- [dioxus, Fullstack GUI library for desktop, web, mobile](https://github.com/dioxuslabs/dioxus)
- [yew, Rust / WASM framework for building client web apps](https://github.com/yewstack/yew)

## Backend Web Framework:
- [axum, Ergonomic and modular web framework built with Tokio, Tower, and Hyper](https://github.com/tokio-rs/axum)
- [actix, Web is a powerful, pragmatic, and extremely fast web framework for Rust](https://github.com/actix/actix-web)

## Fullstack Framework:
- [MoonZoon, Rust Fullstack Framework](https://github.com/MoonZoon/MoonZoon)
- [leptos, Build fast web applications with Rust](https://github.com/leptos-rs/leptos)

## GUI Library:
- [iced, A cross-platform GUI library for Rust, inspired by Elm](https://github.com/iced-rs/iced)
- [sycamore, A library for creating reactive web apps in Rust and WebAssembly](https://github.com/sycamore-rs/sycamore)



# [XCM Format (Dr Wood, Parity)](https://github.com/paritytech/xcm-format)
