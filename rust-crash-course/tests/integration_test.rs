use rust_crash_course::find_golden_proportion;

#[test]
fn golden_integration() {
    let v = find_golden_proportion(3.0);
    assert!((v - 4.85410196625).abs() < 1e-10);
}
