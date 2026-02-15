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
        match val.parse::<f64>() {
            Ok(n) => {
                println!(
                    "Demo: golden proportion of {n} = {}",
                    rust_crash_course::find_golden_proportion(n)
                );
            }
            Err(e) => {
                eprintln!(
                    "Warning: DEMO_VALUE=\"{val}\" is not a valid number ({e}); skipping demo output."
                );
            }
        }
    }
}
