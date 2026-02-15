# Rust Ecosystem and Crates

The ecosystem centers around `crates.io`. Useful crates:

- `serde` for serialization, `reqwest` or `surf` for HTTP, `tokio` for async runtime.
- `anyhow` / `thiserror` for ergonomics in error handling.

Tips:
- Read crate documentation and check maintenance/activity before adding dependencies.
- Keep dependencies minimal for libraries; prefer optional features.
