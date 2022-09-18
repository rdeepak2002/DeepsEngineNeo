# DeepsEngine

![CI Status](https://github.com/rdeepak2002/DeepsEngineNeo/actions/workflows/ci.yml/badge.svg?branch=main) [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

<p align="center">
  <a href="https://github.com/rdeepak2002/DeepsEngineNeo">
    <img src="doc/image/logo.png" height="162" alt="DeepsEngine logo">
  </a>
</p>

## Author

Deepak Ramalingam

## About

Recreation of [DeepsEngine](https://github.com/rdeepak2002/DeepsEngine) in Rust

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) 
- [SDL2](https://wiki.libsdl.org/Installation)
- wasm32-unknown-unknown target for Rust
  - Installation command:
```shell
rustup target add wasm32-unknown-unknown
```
- wasm-bindgen-cli
  - Installation command:
```shell
cargo install -f wasm-bindgen-cli
```

## Common Problems

- [SDL2](https://wiki.libsdl.org/Installation) has undefined symbols on macOS after installing it via [Homebrew](https://brew.sh/)
  - Solution: Add the following to your ```~/.zshrc``` file
```shell
export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib"
```
