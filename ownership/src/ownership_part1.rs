// Ownerchip(Part1): https://medium.com/@thomascountz/ownership-in-rust-part-1-112036b1126b
pub fn run() {
    let hello = "Hello, World";
    let hello1 = hello; // Copy(reference) and Binding
    println!("Copyed: {}, {}", hello, hello1);

    let hello = String::from("Hello World");
    let hello1 = hello; // Move and Binding, Ownership is gone
    println!("Moved: {}", hello1);
    // println!("Moved: {}", hello);  // ERROR
    //
    // To find copy trait, check document.
    // Ownership은 drop trait에의해 제어된다.
    // Drop trait는 destructor와 유사

    let hello = hello1.clone();
    println!("Moved: {}, {}", hello, hello1);
}
