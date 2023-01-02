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
