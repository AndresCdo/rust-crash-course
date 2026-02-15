# Error Handling

Rust has two main error-handling approaches:

- Recoverable errors: `Result<T, E>` with `?` operator.
- Unrecoverable errors: `panic!()` for bugs or unrecoverable states.

Example:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
```

Guidelines:
- Use `anyhow` / `thiserror` for ergonomic error handling in apps/libraries.
- Prefer `Result` in library APIs; reserve `panic!` for truly unrecoverable invariants.
