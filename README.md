# Rust WebAssembly Canvas

This is based on [the Hello Rust example](https://www.hellorust.com/demos/canvas/index.html)
and the [minimal Rust & Wasm example](https://www.hellorust.com/demos/add/index.html).

More resources on Rust & Wasm has since become available:

- https://rustwasm.github.io
- http://fitzgeraldnick.com/2018/12/14/rust-and-webassembly-in-2019.html
- https://github.com/rustwasm/wasm-pack
- https://www.rust-lang.org/what/wasm
- https://rustwasm.github.io/docs/book
- https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
- https://github.com/rustwasm

## Preparing

Install PHP (by default on macOS) or replace file server in `cmd/watch.sh`.

```sh
git clone https://github.com/TomasHubelbauer/rust-wasm-canvas.git
chmod +x cmd/*.sh
cargo install cargo-watch
cargo install --git https://github.com/alexcrichton/wasm-gc
```

Note that `cargo watch` requires `Cargo.toml` to exist.

## Running

(Watch out, this will kill your PHP process, change `cmd/watch.sh` to avoid that.)

- Unix: `./cmd/watch.sh`
- Windows: `cmd/watch.bat`

## To-Do

### Figure out how to use with Cargo and how to depend on crates
