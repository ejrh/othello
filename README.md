Compiling GUI to WASM:

```shell
    cargo build --release --bin gui --target wasm32-unknown-unknown
    wasm-bindgen --out-name gui --out-dir target\for-web --target web target/wasm32-unknown-unknown/release/gui.wasm
```

Copy contents of `target/for-web` to web server.
