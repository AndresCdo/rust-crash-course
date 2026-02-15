# Async and Concurrency

Rust enables fearless concurrency with ownership guarantees and `Send`/`Sync` traits.

- `std::thread` for threads; `async`/`await` with runtimes like `tokio` or `async-std` for IO concurrency.
- Use channels (`std::sync::mpsc`) or `async` channels from `tokio` for communication.

Example — spawn a thread:

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("hello from thread");
});
handle.join().unwrap();
```

For async IO:

```rust
// with tokio runtime
// #[tokio::main]
// async fn main() { .. }
```

Guidelines:
- Prefer simple channels and message passing over shared state.
- Use synchronous code when adequate; async adds complexity.
