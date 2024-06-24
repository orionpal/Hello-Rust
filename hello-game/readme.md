# Hello, game!
## Running
- For local
```cargo run```

- For website

building instructions can be found here: https://bevy-cheatbook.github.io/platforms/wasm/webpage.html

```cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "mygame" \
    ./target/wasm32-unknown-unknown/release/hello-game.wasm
```

## Covers
- bevy

following: https://bevyengine.org/learn/quick-start/getting-started/setup/