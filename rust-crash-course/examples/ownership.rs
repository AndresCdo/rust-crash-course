use rust_crash_course::{borrow_slice, take_ownership};

fn main() {
    let owned = String::from("hello ownership");
    let len = take_ownership(owned);
    println!("Took ownership, length = {}", len);

    let borrowed = "hello borrow";
    let len2 = borrow_slice(borrowed);
    println!("Borrowed slice length = {}", len2);
}
