# Lifetimes

Lifetimes express how long references are valid. They prevent dangling references.

Basics:
- Most lifetime annotations are elided by the compiler.
- You add explicit lifetimes when the compiler cannot infer relationships.

Example:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
```

Guidelines:
- Start by writing code without explicit lifetimes; add annotations when needed.
- Prefer returning owned data (`String`) if lifetime annotations become awkward.
