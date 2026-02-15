fn take_ownership(s: String) -> usize { s.len() }
fn borrow_slice(s: &str) -> usize { s.len() }

fn main() {
    let s = String::from("hello");
    println!("owned len = {}", take_ownership(s));
    let t = "borrow";
    println!("borrowed len = {}", borrow_slice(t));
}
