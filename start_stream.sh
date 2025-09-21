#!/bin/zsh

cargo build -p server
cargo run -p server &

cargo build -p gui
cargo run -p gui 

