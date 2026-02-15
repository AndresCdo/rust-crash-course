# Structs, Enums, and Pattern Matching

Structs group related data; enums model algebraic data types.

```rust
struct Point { x: f64, y: f64 }
enum Message { Quit, Move { x: i32, y: i32 }, Print(String) }

let msg = Message::Move { x: 1, y: 2 };
match msg {
    Message::Move { x, y } => println!("move to {} {}", x, y),
    _ => {}
}
```

Pattern matching is exhaustive; `match` forces handling all cases.
