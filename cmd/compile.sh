mkdir -p bin
rustc +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib src/index.rs -o bin/index.wasm
cd bin
wasm-gc index.wasm index.min.wasm
