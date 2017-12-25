# Rust WebAssembly Canvas

## Preparing

Install PHP (by default on macOS) or replace file server in `cmd/watch.sh`.

```sh
git clone https://github.com/TomasHubelbauer/rust-wasm-canvas.git
chmod +x cmd/*.sh
cargo install cargo-watch
cargo install --git https://github.com/alexcrichton/wasm-gc
```

## Running

(Watch out, this will kill your PHP process, change `cmd/watch.sh` to avoid that.)

- Unix: `./cmd/watch.sh`
- Windows: `cmd/watch.bat`
