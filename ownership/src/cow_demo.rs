// CoW: Copy On Write: https://medium.com/@cort3x/what-is-cow-in-rust-af56b3ef0a5c
use std::borrow::Cow;

pub fn run() {
    // let mut s = String::from("hello").as_str();
    let s = "Cow: Copy on Write";
    to_uppercase_one(s);
    to_uppercase_two(s);
}

// Borrowed variant, this means that the cow instance does not own the data and cannot modify it, it provides a read-only view of the data.
// Owned variant, this means the Cow instance owns the data and can modify it. However, the data is not immediately copied; instead, it is only copied when a modification is attempted.
fn to_uppercase_one(s: &str) -> Cow<str> {
    if s.chars().any(|c| c.is_lowercase()) {
        Cow::Owned(s.to_uppercase()) // <-- Allocation
    } else {
        Cow::Borrowed(s)
    }
}

fn to_uppercase_two(s: &str) -> String {
    if s.chars().any(|c| c.is_lowercase()) {
        s.to_uppercase() // <-- allocation
    } else {
        s.to_string() // <-- allocation
    }
}
