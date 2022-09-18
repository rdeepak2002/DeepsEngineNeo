#!/bin/sh
cargo build --target wasm32-unknown-unknown --verbose
rm -rf ./generated
mkdir -p ./generated
wasm-bindgen ./target/wasm32-unknown-unknown/debug/DeepsEngineNeo.wasm --out-dir ./generated --target web
cp index.html generated