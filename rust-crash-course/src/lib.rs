//! Core library utilities used by the course examples and exercises.
//!
//! Keep this file small and well-documented so students can read it
//! quickly and run tests against it.

/// Returns the golden proportion of `n` (n * φ).
///
/// φ (phi) is the golden ratio constant ≈ 1.61803398875.
pub fn find_golden_proportion(n: f64) -> f64 {
    const GOLDEN_RATIO: f64 = 1.61803398875_f64;
    n * GOLDEN_RATIO
}

/// Small helper demonstrating ownership vs borrowing.
pub fn take_ownership(s: String) -> usize {
    s.len()
}

/// Borrow a string slice and return its length (no ownership taken).
pub fn borrow_slice(s: &str) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn golden_basic() {
        let out = find_golden_proportion(2.0);
        assert!((out - 3.2360679775).abs() < 1e-10);
    }

    #[test]
    fn ownership_vs_borrow() {
        let s = String::from("abc");
        let l = take_ownership(s);
        assert_eq!(l, 3);
        let t = "xyz";
        assert_eq!(borrow_slice(t), 3);
    }
}
