#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

pub fn run() {
    let my_point = Point::new(1, 2);
    println!("My point: {:?}", my_point);
}
