Works on both Windows and Mac

## Covered Topics
- Setting up Rust
- Basics of creating a Rust project
    - main function
    - modules
    - printing to console
    - reading input from console
- Compiling Rust project
- See project folders for examples

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

## Trying out the examples
In each project folder there is a readme with instructions on what's covered and the usage. Make sure you're in that directory when running commands. There are comments in the code to try and lead the logic a bit


## Basics
rust file extension is .rs

make sure dependencies are built: ```cargo build```

to compile a file, type this into console:

```rustc filename.rs```

for example,

```rustc src/main.rs```

This creates an executable file.

If there are dependencies you should run:

```cargo build```

And the executable will be found in target/debug and will be the name of the project


and then running:
./filename

will execute the file



## Crates to be explored
- Criterion - benchmarking
- Rayon - parallelism
- Atomic - numbers
- Axum - back end api
- egui - simple visuals
- bevvy - game engine