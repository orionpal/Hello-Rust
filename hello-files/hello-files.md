## Covers
- reading and writing common file types
    - json (using serde crate)
    - csv (using csv crate)
    - txt (using vanilla rust)
- slightly more complicated project structure
    - modules within modules
- running/compiling using cargo instead of rustc

## Take aways
- When returning a Result that may or may not have worked, use "let value = result?;". the "?" either returns the successfully retrieved object or returns the error and exits the function
- You can also use "match" when retrieving a value. matching to "Ok(val)" lets you directly use the val
- When organizing your code, mod.rs should exist in the directories to define what's being shared

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
- csv