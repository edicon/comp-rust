
pub fn run() {
    println!("Ownership Part2:");

    let name = "Rust";
    let version = 1.60;
    let year = 2021;

    // {} 형식 지정자 사용
    println!("Hello, {}. Welcome to Rust version {} released in {}!", name, version, year);

    // {:?} 형식 지정자 사용
    println!("Debug: pointer={:p} name={:?}, version={:?}, year={:?}", name, name, version, year);

    let number = 42;

    // {:*} 형식 지정자 사용
    // println!("The value of number is {:5*}", number);  // Compile error
    println!("The value of number is {:5}", number);
}
