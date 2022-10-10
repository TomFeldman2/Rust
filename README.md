# Rust
Learning Rust using "The Book"

## Installation
- ```curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh```
- rustup update
- rustup self uninstall

## Cargo
- cargo build
- cargo run
- cargo update
- cargo doc --open

## Ownership Rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## The Rules of References
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

## ForMe
- String vs str
- drop
- move vs copy
- a reference’s scope starts from where it is introduced and continues through the last time that reference is used

