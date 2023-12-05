// Ownership and Function Parameter
pub fn run() {
    println!("Ownership Part2:");
    ownership_func1();
    ownership_func2();
    giving_ownership();
    give_take_ownership();
    reference_borrowing();
    mutability();
    dangling_reference();
}

fn ownership_func1() {
    let string = "Copy Ownership";
    println!("{:p}", string);

    // Literal Type: Copy to stack and bind
    copy_ownership(string);
}
fn copy_ownership(string: &str) {
    println!("{}: {:p}", string, string)
}

fn ownership_func2() {
    let string = String::from("Moving ownership: ");
    println!("{:p}", string.as_ptr());

    // Move ownership to bar
    move_ownership(string.clone());
    move_ownership(string);

    // Error because of ownership
    // println!("{}", string);
}
fn move_ownership(string: String) {
    println!("{}: {:p}", string, string.as_ptr())
}

fn giving_ownership() {
    let string = take_ownership();
    println!("Checking Ownership: {:p}", string.as_ptr());
}

fn take_ownership() -> String {
    let string = String::from("Taking Ownership");
    println!("{}: {:p}", string, string.as_ptr());
    return string;
}

fn give_take_ownership() {
    let string = String::from("Give & Take Ownership");
    println!("{}: {:p}", string, string.as_ptr());
    give_take(string);
}

fn give_take(string: String) -> String {
    let string = String::from("Give & Take");
    println!("{}: {:p}", string, string.as_ptr());
    return string;
}

fn reference_borrowing() {
    let string = String::from("Reference and Borrowing");
    println!("{}: {:p}", string, string.as_str());

    borrowing(&string); // Borrowing: reference of  String type
}

// Borrowing: reference of  String type
// No ownership but only borrowed
// Ampersands indicate references, which allow the passing of values without giving up ownership!
fn borrowing(string: &String) {
    println!("Borrowing: {}, {:p}, {:p}", string, string, string.as_str());
    nested_borrowing(string);
}
fn nested_borrowing(string: &String) {
    println!(
        "Nested Borrowing: {}, {:p}, {:p}",
        string,
        string,
        string.as_str()
    );
}

fn mutability() {
    let mut string = String::from("Mutability");
    println!("{}: {:p}, {:p}", string, &string, string.as_ptr());
    string.push_str(" added for mutability");
    mutability_foo(&mut string);
}

fn mutability_foo(string: &mut String) {
    println!("{}: {:p}, {:p}", string, &string, string.as_ptr());
    string.push_str(" added for mutability(nested)");
}

fn dangling_reference() {
    let string = dangling();
}

fn dangling() -> String {
    let string = String::from("Dangle String");
    println!("{}, {:p}", string, &string);
    string // Move Ownership: No Error
           // &string     // Dangling Error: borrowing
}
