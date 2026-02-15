# Performance and Safety

Rust gives low-level control with safety guarantees. Key points:

- Prefer iterators and zero-cost abstractions — they compile to efficient code.
- Use `cargo bench` (or Criterion) for benchmarking.
- `unsafe` lets you bypass some checks; keep it minimal and well-audited.

When optimizing:
- Measure first (profiling), then optimize hot paths.
- Prefer algorithmic improvements over micro-optimizations.
