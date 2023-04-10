#![deny(clippy::all)]
// Define constants (immutable)
const GOLDEN_RATIO: f64 = 1.61803398875;

fn main() {
    /* This is the main function. */

    // Variables
    let name: &str = "Andres";
    let age: f64 = 25.0;

    // Print
    println!("Hello, {}! You are {} yeras old.", name, age);
    print!(
        "Your golden ratio is: {} years old.",
        find_golden_proportion(&age)
    );
}

fn find_golden_proportion(&float_number: &f64) -> f64 {
    /* This function finds the golden proportion of a number. */
    float_number * GOLDEN_RATIO
}
