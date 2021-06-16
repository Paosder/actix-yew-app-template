#!/bin/zsh
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli