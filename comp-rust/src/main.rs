use std::fs::File;
// use std::io::{Error};

mod traits;

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

    lifetime_function();
    lifetime_data();

    // Day2
    pattern_matching();
    struct_method();
    destructuring_enums();

    if_let_express();
    while_let_express();

    option_result();
    option_result2();

    heap_allocation();

    traits::run();
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
    let s2: &[i32] = &a[2..4]; // Copied
    println!("a: {a:?}, s: {s:?}");

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


// ove Semantics
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
fn add2(p1: &Point, p2: &Point) -> Point {
    let p3: Point = Point(p1.0 + p2.0, p1.1 + p2.1);
    p3
}
// Borrowing
fn borrowing()  {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");
    let p4 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p4:?}");

    // Closer
    let add3 = |p1: &Point, p2: &Point| -> Point {
        Point(p1.0 + p2.0, p1.1 + p2.1)
    };
    add3(&p1, &p2);
}


fn shared_unique_borrow() {
    let mut a: i32 = 10;
    let b: &i32 = &a;
    // let c: &i32 = &a;

    {
        // let c: &mut i32 = &mut a;
        // *c = 20;
    }

    println!("a: {a}");
    println!("b: {b}");
}

// Lifetime in function: &'a, &'ab, &'doc like as named register
fn lifetime_left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn lifetime_function() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);
    let p3: &Point = lifetime_left_most(&p1, &p2);
    println!("left-most point: {:?}", p3);
}

// fn no_lifetime_left_most(p1: &Point, p2: &Point) -> &Point {
//     if p1.0 < p2.0 { p1 } else { p2 }
// }
//
// fn  no_lifetime_function () {
//     let p1: Point = Point(10, 10);
//     let p2: Point = Point(20, 20);
//     let p3: Point =  no_lifetime_left_most( &p1, &p2);
// }

// Lifetime in data structure &'doc
#[derive(Debug)]
// struct Highlight<'doc>(&'doc str);
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}");
}

fn lifetime_data() {
    let text =  String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    // erase(text);  // can't move because of borrowed
    println!("{fox:?}");
    println!("{dog:?}");
}


// Day2
//  Struct:: ethod
#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Race {  // No receiver, a static method
        Race { name: String::from(name), laps: Vec::new() }
    }

    fn add_lap(&mut self, lap: i32) {  // Exclusive borrowed read-write access to self
        self.laps.push(lap);
    }

    fn print_laps(&self) {  // Shared and read-only borrowed access to self
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {  // Exclusive ownership of self
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn struct_method() {
    let mut race = Race::new("onaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();

    // race.add_lap(71);
    // race.print_laps();
}

// Pattern matching like as  switch

fn pattern_matching() {
    let input = 'x';

    match input {
        'q'         => println!("Quiting"),
        'a' | 's'   => println!("Quiting"),
        '0'..='9'   => println!("Number"),
        _           => println!("Something else"), // _: wildcard
    }
}


enum ResultMe {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> ResultMe {
    if n % 2 == 0 {
        ResultMe::Ok(n / 2)
    } else {
        ResultMe::Err(format!("cannot divide {} into two equal parts", n))
    }
}

fn destructuring_enums() {
    let n = 100;
    match divide_in_two(n) {
        ResultMe::Ok(half) => println!("{n} divided in two is {half}"),
        ResultMe::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}


fn if_let_express() {
    let arg = std::env::args().next();
    if  let Some(value)  = arg {
        println!("Program name {value}");
    } else   {
        println!("Missing name?");
    }

}

fn while_let_express() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
}

fn option_result() {
    // let will_overflow: Option<u8> = 10_u8.checked_add(200);
    let will_overflow: Option<u8> = 10_u8.checked_add(250);
    match will_overflow {
        Some(sum) => println!("interesting: {}", sum),
        None => eprintln!("addition overflow"),
    }
}

//https://ferrous-systems.github.io/teaching-material/control-flow.html#/_using_option_and_result_2
fn option_result2() {
    let file_open: Result<File, std::io::Error> = File::open("Does noet exist");

    match file_open {
        Ok(f) => println!("File: {:?}", f),
        Err(e) => eprintln!("Open failed: {:?}", e),
    }
}

fn heap_allocation() {
    let point = Point (1, 1);
    let point_on_heap = Box::new(point);
    println!{"point: {:?}, on_heap: {:?}", point, point_on_heap};
}
