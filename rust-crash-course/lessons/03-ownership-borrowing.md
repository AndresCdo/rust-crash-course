# Ownership and Borrowing

Ownership is Rust's core memory-safety concept. Key points:

- Each value has a single owner (a variable).
- When the owner goes out of scope, the value is dropped.
- Move semantics: assigning or passing `String` moves ownership by default.
- Borrowing allows temporary access without transfer of ownership via `&T` (immutable) or `&mut T` (mutable).

Example — borrowing vs moving:

```rust
let s = String::from("hello");
let r = &s; // borrow immutably
println!("{}", r);
// s still usable here

let s2 = String::from("world");
let moved = s2; // move: s2 is no longer valid
```

Rules to remember:
- At any time, either one mutable borrow or any number of immutable borrows.
- References must point to valid data; no dangling references.
