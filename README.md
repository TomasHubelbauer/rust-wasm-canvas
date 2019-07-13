# Rust WebAssembly Canvas

This is based on [the Hello Rust example](https://www.hellorust.com/demos/canvas/index.html)
and the [minimal Rust & Wasm example](https://www.hellorust.com/demos/add/index.html).

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

Figure out how to use with Cargo and how to depend on crates.
