#!/bin/bash
echo "Ensuring WebAssembly target is installed..."
rustup target add wasm32-unknown-unknown

if ! command -v wasm-bindgen &> /dev/null
then
    echo "Installing wasm-bindgen-cli (first time only, this takes ~60s)..."
    cargo install wasm-bindgen-cli --version 0.2.93
fi

echo "Building WebAssembly Production Engine..."
cargo build -p demo_app --lib --release --target wasm32-unknown-unknown --features client || exit 1

echo "Running wasm-bindgen..."
wasm-bindgen ../../../target/wasm32-unknown-unknown/release/demo_app.wasm --out-dir ./ --target web --no-typescript --out-name wasm_app

echo "BUILD COMPLETE! Upload index.html and wasm_app* files to any static host!"
