fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "2.0".into());
    let n: f64 = arg.parse().unwrap_or(2.0);
    const GOLDEN: f64 = 1.61803398875;
    println!("Golden proportion of {} = {}", n, n * GOLDEN);
}
