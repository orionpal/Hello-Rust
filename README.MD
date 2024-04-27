DONE ON WINDOWS 10
soon to be tested on mac

## Covered Topics
- Setting up Rust
- Basics of creating a Rust project
    - main function
    - modules
    - printing to console
    - reading input from console
- Compiling Rust project

## Setting up Rust
1. download the rust installer here:
https://www.rust-lang.org/tools/install
2. follow installer instructions
3. Congrats! You should now have access to:
    - rust compiler command: **rustc**
    - rust package manager: **cargo**
    - rust toolchain (used for switching rust versions): **rustup**
    - * if you can't run ```rustc --version``` then you may need to add your ~./cargo to PATH or ENVIRONMENT

## Starting your project
1. Open a terminal/console in the directory where you want your project to be created
2. run in terminal:
    - ```cargo new {project_name}```
3. You should now have a very basic project structure:
    - **Cargo.toml**: handles dependencies
    - **src**: directory to contain source code
    - **src/main.rs**: located in src and will be what is run when you compile the project

following tutorial:
https://doc.rust-lang.org/rust-by-example/hello.html
https://doc.rust-lang.org/rust-by-example/meta/doc.html

rust file extension is .rs

make sure dependencies are built: cargo build

to compile a file, type this into console:
rustc filename.rs
This creates an executable (.exe) file


and then running:
./filename



## Some stuff
Criterion - benchmarking app
Atomic - numbers?
Yew - webassembly packaging
rocket - framework for web api
