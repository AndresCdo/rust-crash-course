use rust_crash_course::find_golden_proportion;
use std::env;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "2.0".to_string());
    let n: f64 = arg.parse().unwrap_or(2.0);
    println!("Golden proportion of {} = {}", n, find_golden_proportion(n));
}
