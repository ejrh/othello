Compiling GUI to WASM:

```shell
    cargo build --release --bin othello_gui --target wasm32-unknown-unknown
    wasm-bindgen --out-name othello_gui --out-dir target\for-web --target web target/wasm32-unknown-unknown/release/othello_gui.wasm
```

Copy contents of `target/for-web` to web server.
