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

## Common Problems

- SDL2 not working on MacOS after installing it via brew
  - Solution: Add the following to your ~/.zshrc file
```shell
export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib"
```
