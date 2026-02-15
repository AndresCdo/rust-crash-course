# Testing, Tooling, and Cargo

Cargo is Rust's package manager and build tool.

Commands:
- `cargo build`, `cargo run`, `cargo test`, `cargo doc --open`.

Testing:
- Unit tests: inside `#[cfg(test)] mod tests {}` in library files.
- Integration tests: files under `tests/` directory treated as separate crates.

Example unit test (see `src/lib.rs`):

```rust
#[cfg(test)]
mod tests { #[test] fn it_works() { assert_eq!(2+2, 4); } }
```

Tooling:
- `cargo clippy` (lints), `cargo fmt` (formatting), `rust-analyzer` for IDEs.
