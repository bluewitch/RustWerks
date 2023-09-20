# Rust_QA.md

# How did you hear about Rust?
Well, it's a bit of a journey down memory lane, but I'm glad you asked. Back in the day, I spent a considerable amount of time delving deep into various forums, GitHub repositories, and open-source projects. My primary interest was always optimizing performance and ensuring utmost security. I've always believed that efficient software shouldn't have to compromise on safety and vice versa.

One evening, while traversing the labyrinth of a particular forum known for its forward-thinking discussions, I stumbled upon a heated debate about modern programming languages and their ability to handle concurrency without sacrificing safety. That was the first time I heard about Rust.

The idea of a language that promised both performance and memory safety, without a garbage collector, intrigued me. It sounded almost too good to be true. But as I dove into the Rust Book and started tinkering with the language, I realized it wasn't just hype. Rust's ownership model and its 'borrow checker' were revolutionary. The compiler, affectionately named 'rustc,' became my strictest teacher and my closest ally, guiding me away from potential pitfalls and ensuring my code was safe and efficient.

The more I delved into Rust, the clearer its advantages became, especially when working on projects where concurrency, low-level systems programming, or embedded software were involved.

So, in a sense, it was a combination of my insatiable curiosity, being in the right place at the right time, and Rust's compelling value proposition that led me to it. From then on, I've adopted Rust in various projects, and it's been an exhilarating journey of discovery and mastery.

# Why did you choose to program in Rust?
Ah, the age-old 'why Rust' question. It's one of my favorites. Dive into any deep technical conversation, and you'll often find passionate discussions on tool choices. But Rust, for me, was more than just another tool—it was a revelation.

You see, as someone who has always been on the bleeding edge of technology, pushing the boundaries of what's possible, I've faced my fair share of challenges. Be it battling with arcane segmentation faults, race conditions, or just the sheer frustration of manual memory management in other languages, I've been through the gauntlet.

Enter Rust. The first time I truly grappled with its unique ownership system and experienced the 'helpful' nudges of the borrow checker, it was like a lightbulb moment. Here was a language that wasn't just content being fast or being safe—it aspired to be both. And in our line of work, where every nanosecond and every byte can make a world of difference, that's a game-changer.

Moreover, as someone who's dabbled in the darker alleys of the cyber world, security is paramount to me. Rust's safety guarantees and its emphasis on zero-cost abstractions meant that I didn't have to trade off performance for security. It offered a robust ecosystem with a community that was as obsessed with safety and efficiency as I was.

But the clincher? It's the culture. The Rust community is open, inclusive, and constantly evolving. It embodies the ethos of 'fearless concurrency.' And to be perfectly honest, the joy of seeing a piece of Rust code compile after a dance with the borrow checker—it's unparalleled.

So, in essence, I didn't just 'choose' Rust. It was an alignment of philosophy, necessity, and the promise of crafting something efficient, safe, and beautiful. It was a no-brainer.


# Any previous experience with model driven development or Model checking? functional programming? embedded?
Ah, indeed, you've touched on some of the most intriguing corners of my journey. Let's break it down.

Model-Driven Development (MDD) & Model Checking: Absolutely. My forays into the deeper realms of computer science and hacking naturally led me to appreciate the elegance of formal methods. Model-driven development, in particular, caught my attention when I was exploring ways to ensure the correctness and reliability of systems without the usual exhaustive testing. By defining system behavior with models and then generating code from those models, I found we could create systems with predictable behavior, especially in mission-critical applications. Model checking, on the other hand, offered a way to mathematically prove the correctness of those models. Over the years, I've used tools like TLA+ and Spin, and I've been closely following the advancements in Rust's formal verification scene too.

Functional Programming (FP): Oh, where do I begin? My intrigue with FP started long before Rust, with languages like Haskell and Erlang. The purity, immutability, and mathematical rigor of functional programming appealed to the part of my brain that thrived on solving intricate problems. It introduced me to concepts like monads, functors, and pure functions. And while Rust isn't purely functional, its emphasis on immutability and its pattern matching capabilities always gave me that familiar FP vibe. I often find myself leaning on functional paradigms even within Rust, making code more declarative and easy to reason about.

Embedded Systems: Now that's a realm where Rust truly shines. I've had a penchant for embedded systems since my earlier hacking days—there's something thrilling about directly manipulating hardware. Rust's zero-cost abstractions and safety guarantees made it an ideal choice for such work. I've worked on several projects, from custom IoT devices to safety-critical systems, where the low-level control of C and C++ was needed, but without their inherent risks. Rust provided that middle ground, where I could work close to the metal without constantly fearing memory errors or data races.

So, in a nutshell, yes—my journey has intertwined with these domains quite intricately. Each experience, each paradigm, has only enriched my understanding and enhanced my ability to tackle complex problems head-on.


- C/C++ or other real time capable language?
- Projects worked on? Any open source?
- team size?
- University students: which lectures taken?
- Version control and Continuous Integration experience?




1) Explain Rust?
Rust is blazingly fast systems programming language that prevents segfaults and guarantees thread safety.

2) Rust was Designed by whom
Originally Rust was designed by Graydon Hoare, Now it managed by Rust project developers.



3) When the first version of Rust was released
The first version of Rust was released in the year 2010.




4) Rust syntax is similar to which programming Language
Rust is syntactically similar to C++.



5) List some features of Rust?
Rust Programming Language comes with following features Sets.
zero-cost abstractions
move semantics
guaranteed memory safety
threads without data races
trait-based generics
pattern matching
type inference
minimal runtime
efficient C bindings



6) Who uses Rust?
Google
360dialog
OneSignal
Coursera
Atlassian
Braintree
npm, Inc
Mozilla
Academia.edu
Xero

7) List the Platforms supported by Rust Programming Language
Linux, Mac, and Windows, on the x86 and x86-64 CPU architecture, are some major platforms supported by Rust Programming Language. 



8) List steps to install Rust?
On Linux and macOS simply open the terminal and run following commands
$ curl https://sh.rustup.rs -sSf | sh
Above command will download a script, and start the installation process. If everything was good and no error occurred you will see below success message.
Rust is installed now. Great!
If you are on Windows. Installing Rust is very easy just download and run rustup-init.exe File




9) Do you remember which command is used to uninstall Rust?
$ rustup self uninstall command is used to uninstall Rust programming language.



10) How to get installed the version of Rust?
rustc –version command is used to get installed version of Rust.




11) How to write and run a Rust Program?
Step to create and run a Rust Program
create a file name main.rs and add following code in it.
fn main() {
    println!("Hello, Rust!");
}
On Linux or macOS to run open terminal run below command

$ rustc main.rs
$ ./main




12) Explain Cargo?
Cargo is Rust’s build system and package manager, and Rustaceans use Cargo to manage their Rust projects.Cargo manages three things: building your code, downloading the libraries your code depends on, and building those libraries.



13) What Is That Cargo.lock?
When we run cargo build command it creates a new file called Cargo.lock.Cargo uses the Cargo.lock file to keep track of dependencies in your application.



14) What cargo new command do?
cargo new creates a new project with Cargo. Below is the syntax to create a sample project using Rust Cargo.
 $ cargo new project_name --bin



Oleksii

Strong fundimentals, Linux skills knows Rust core, and has implimented rust specific projects from other languages.  A big plus.

Intergrates CI/CD in his work, very open to converstations and dialog to fix issues, concearn about the work, not about the ego good to work with

Would like to see more on his Substrate, specifically with pallets (what we are working with), we can teach this he is very comunicatable.

Good for a junior role, and can bring to mid level quickly.  Strong fundimentals.


Dusan

He is comming from Ethereum, solidity and has been working on professional projects related to Ethereum for the energy sector.

He is very enthusiastic about going more into Substrate and Rust, but is abit new in the Substrate and Rust department.  Has solid first principles, but does not intergrate CI/CD as much in his internal methdologies. 

Good area knowledge of Blockchain, but I am unclear of his Rust specific focus.


