# Iterators and Collections

Rust's iterator adapters are powerful and composable.

Collections:
- `Vec<T>`, `HashMap<K,V>`, `HashSet<T>`, `LinkedList<T>` (rare).

Example — iterator chain:

```rust
let nums = vec![1,2,3,4];
let s: i32 = nums.iter().map(|x| x * 2).filter(|x| x % 3 != 0).sum();
println!("sum = {}", s);
```

Tip: favor iterators and slices over manual indexing for safety and expressiveness.
