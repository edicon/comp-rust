use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    println!("Hello, world!");

    //  두개의 string을 비교하는 함수
    let s1 = String::from("hello");
    let s2 = "hello";

    let result = longest(s1.as_str(), s2);
    println!("The longest string is {}", result);

    // how to write files in rust
    // - https://oliverjumpertz.medium.com/how-to-write-files-in-rust-d215f54e91b9
    let mut f = File::create("hello.txt").expect("Failed to create file");
    f.write_all(b"Hello, world!").expect("Failed to write to file");

    // how to read files in rust
    let mut f = File::open("hello.txt").expect("Failed to open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Failed to read file");
    println!("File contents: {}", s);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
