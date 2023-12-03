# Advent of Code 2023

## Introduction

Code written in Rust to solve the puzzles of the Advent of Code 2023 event.

More info at: [Advent of Code 2023](https://adventofcode.com/2023/about)

## Dev

> NOTE: Install cargo watch with `cargo install cargo-watch`.

```sh
# Terminal 1 - To run.
cargo run
# Or
cargo watch -q -c -w src/ -x "run"
```

## Unit Test

```sh
cargo test -- --nocapture

cargo watch -q -c -x "test -- --nocapture"
```
