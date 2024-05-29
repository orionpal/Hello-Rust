## Covers
- reading and writing common file types
    - json (using serde crate)
    - csv 
    - txt (using vanilla rust)
- slightly more complicated project structure
    - modules within modules
- running/compiling using cargo instead of rustc

## Running
Try using:

```
cargo build
```

This is necessary because this project has dependencies which you can find in Cargo.toml

After compiling, the executable file will be seen in /target/debug/ and will be names the project-name which in this case is hello-files.

Then you can run with one of the examples in the examples folder, for example:

```
cd target/debug
./hello-files ../../src/examples/example.txt
```

## Crates Used
- serde
- serde_json