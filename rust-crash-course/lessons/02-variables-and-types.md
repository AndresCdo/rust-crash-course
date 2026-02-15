# Variables and Types

Rust is statically typed: every value has a type known at compile time.

- Immutable by default. Use `let mut` for mutable bindings.
- Type inference is strong: `let x = 5;` infers `i32` by default.
- Primitive numeric types: `i8..i128`, `u8..u128`, `f32`, `f64`.
- Common compound types: tuples `(i32, &str)`, arrays `[i32; 3]`, slices `&[i32]`.

Example:

```rust
let x = 42; // i32 inferred
let mut s = String::from("hello");
s.push_str(" world");
```

Tips:
- Prefer immutable bindings; mutate only when necessary.
- Use explicit types in public APIs for clarity.
