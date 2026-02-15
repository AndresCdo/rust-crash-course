# Generics and Traits

Generics let you write code that works for many types. Traits express shared behavior.

Example — generic function and trait bounds:

```rust
fn duplicate<T: Clone>(v: T) -> (T, T) { (v.clone(), v) }

trait Greeter { fn greet(&self); }
impl Greeter for &str { fn greet(&self) { println!("hello {}", self); } }
```

When designing APIs:
- Prefer small, focused traits.
- Use generic type parameters for performance and flexibility.
