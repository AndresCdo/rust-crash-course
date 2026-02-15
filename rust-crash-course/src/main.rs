//! A small demo binary for the Rust Crash Course repository.
//!
//! This binary demonstrates a tiny part of the course: using the
//! `rust_crash_course` library and printing results. The real content
//! of this repository is in the `lessons/`, `exercises/` and `examples/`
//! directories.

fn main() {
    println!("Rust Crash Course — welcome!");
    println!();
    println!("See the lessons in the lessons/ folder and run examples:");
    println!("  cargo run --example golden");
    println!("  cargo run --example ownership");
    println!();
    // Small demo using the library function
    if let Ok(val) = std::env::var("DEMO_VALUE") {
        if let Ok(n) = val.parse::<f64>() {
            println!("Demo: golden proportion of {n} = {}", rust_crash_course::find_golden_proportion(n));
        }
    }
}
