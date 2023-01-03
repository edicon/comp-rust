fn main() {
    let mut x: i32 = 6;
    print!("{x}");
    while x != 1 {

        if x % 2 == 0{
            x = x / 2;
        } else  {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();

    type_array();
    reference();

    // dangle_ref():
    slices();
    type_string();

    method();
    implicit_conv();

    array_loop();
    variables();
    type_inference();
    compute_digest("Hello");
    scope_shadowing();

    move_semantic();
    move_in_func();
    copy_cloning();
    borrowing();
    shared_unique_borrow();
}

fn type_array() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a);

    let t: (i8, bool) = (7, true);
    println!("1nd Index: {}, {:?}", t.0, t);
    println!("2nd Index: {}, {:?}", t.1, t);
}

fn reference() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");
}

fn dangle_ref() {
    let ref_x: &i32;
    // {
    //     let x: i32 = 10;
    //     ref_x = &x;
    // }
    // println!("ref_x: {ref_x}");
}

fn slices() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}, {:?}", a);

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");

    // let ss: [i32; 2] = &a[2..4];
    // println!("s: {s:?} {ss:?}");
}

fn type_string() {
    let s1: &str = "Hello";                      // &str: immutable
    println!("s1: {s1}, {}, {s1:?}", s1);

    let mut s2: String = String::from("Hello "); // mutable
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn method() {
    let mut rect = Rectangle {width: 10, height: 5};
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}


fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn implicit_conv() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));

}

fn array_loop() {
    let array = [10, 20, 30];
    println!("array: {array:?}");

    for n in array {
        print!(" {n}");
    }
    println!();

    for i in  0..3 {
        print!(" {}", array[i]);
    }
}

fn variables() {
    let x: i32 = 10;
    println!("x: {x}");

    // immutable  by default
    // x = 20;
    // println!("x: {x}");
}

// Type Inference
fn take_u32(x: u32) {
    println!("u32: {x}");
}

fn take_i8(y: i8) {
    println!("i8: {y}");
}

fn type_inference() {
    let x = 10;
    let y = 20;

    take_u32(x);
    take_i8(y);
    take_u32(y.try_into().unwrap());
}

// Static and Constant Variables
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    println!("Digest: {digest:?}");
    digest
}

// Scope and Shadowing
fn scope_shadowing() {
    let a = 10;
    println!("Before: {a}");

    {
        let a = "Hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shodowed in inner scopr scope: {a}");
    }
    println!("After: {a}");
}


// Move Semantics
//  -move ownership
fn move_semantic() {
    let s1: String = String::from("Hello");
    let s2: String =  s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}

fn say_hello(name: String)  {
    println!("Hello {name}");
}

fn move_in_func() {
    let name =  String::from("Rust");
    say_hello(name);
    // say_hello(name);
}


#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn copy_cloning() {
    let p1 = Point(3, 4);
    let p2 = p1;
    let p3 = p1.clone();

    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
    println!("p3: {p3:?}");
}


fn add(p1: &Point, p2: &Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}
// Borrowing
fn borrowing()  {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");
}

fn shared_unique_borrow() {
    let mut a: i32 = 10;
    let b: &i32 = &a;

    // {
    //     let c: &mut i32 = &mut a;
    //     *c = 20;
    // }

    println!("a: {a}");
    println!("b: {b}");
}
