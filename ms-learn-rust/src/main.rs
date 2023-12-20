fn main() {
    let mut a_num: i32 = 0;
    let a_word = "Ten";

    a_num = 10;

    println!("The Number is {}", a_num);
    println!("The word is {}", a_word);

    shadow_variable();
    string_type();
    arrays();
    vectors();
    hash_map();
    some_none();

    handle_error();
}

// shadow: mutable 선언 없이 변수를 변경, 
// - not mutable
fn shadow_variable() {
    let shadow_num = 5;
    let shadow_num = shadow_num + 1;
    let shadow_num = shadow_num * 2;

    // shadow_num = 10; // error: cannot assign twice to immutable variable

    println!("The value of shadow_num is: {}\n", shadow_num);
}

// String
// - &str: string literal, stack에 저장
// - String: heap에 저장
fn string_type() {
    let character_1: char = 's';
    let character_2: char = 'f';

    let smiley_face = '😃';
    let string_1 = "miley ";

    let string_2: &str = "face";
    // let string_2: str = "face";

    println!("{} is {}{}{}{}\n", smiley_face, character_1, character_2, string_1, string_2);
}

// Arrays
// - [T; N] T: type, N: length, size는 고정
fn arrays() {
    let array_1 = [1, 2, 3];
    let array_2: [i32; 3] = [1, 2, 3];
    let array_3 = [3; 3]; // [3, 3, 3]

    println!("The first element  of array_1 is: {}", array_1[0]);
    println!("The second element of array_2 is: {}", array_2[1]);
    println!("The third element  of array_3 is: {}\n", array_3[2]);

    // out of array
    // let array_5 = array_1[5];
    // println!("The sixth element of array_1 is: {}", array_1[5]); // error: index out of bounds
}

// tuple, struct, enum, union에 사용
// - #[derive(Debug)]

// Vectors
// - Vec<T> T: type, size는 가변
// - Vec<T>는 heap에 저장, array는 stack에 저장
// - vec! macro를 사용하여 생성 : vec![1, 2, 3, 4, 5]
fn vectors() {
    let vector_1 = vec![1, 2, 3];
    let vector_2: Vec<i32> = vec![1, 2, 3];
    let vector_3 = vec![3; 3]; // [3, 3, 3]

    println!("The first element  of vector_1 is: {:?}", vector_1[0]);
    println!("The second element of vector_2 is: {:?}", vector_2[1]);
    println!("The third element  of vector_3 is: {:?}", vector_3[2]);

    // out of array, 
    // - array와 달리 범위를 벗어나는 경우를 방지할 수 없다.
    // let vector_5 = vector_1[5];
    // println!("The sixth element of vector_1 is: {:?}", vector_1[5]); // error: index out of bounds
    
    // Create empty vector, declare vector mutable so it can grow and shrink
    // let mut vector_t: Vec<T> = Vec::new();
    let mut vector_4: Vec<String> = Vec::new();
    println!("The vector_4 is: {:?}", vector_4);

    // push/pop
    // vector_4.push(1); // compile error
    // vector_4.push("Hello"); // compile error
    vector_4.push("Hello world".to_string());
}

// Hash Map
// -hash.insert(key, value)
// -hash.remove(key)
// -hash.get(key): Some(), None
fn hash_map() {
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

    // Look for a specific review
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));
}

fn some_none() {
    match divide(10, 2) {
        Some(result) => println!("나누기 결과: {}", result),
        None => println!("나누기 실패"),
    }
}

// 함수의 return값으로 Option<T>
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn handle_error() {
    // panic_macro();
    // panic_array();
    option_enum();
    result_enum();
}

// #[warn(dead_code)]
fn panic_macro() {
    panic!("crash and burn");
}

fn panic_array() {
    let v = vec![1, 2, 3];

    v[99];
}

// 값이 있거나 없는경우
// - 함수의 return 값으로 사용: Oprion(String): 선택적 문자열(있어가 없거나)
// emum Option<T> {
//    Some(T),
//    None,
// }
// Vec::get(index): Oprion<T>: Some(&T), None
fn option_enum()  {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }
}

// if let Pattern : 단일 Pattern만 사용 가능
// - if let Some(x) = a_value { ... }
//
// unwrap: Option<T>의 내부 값(T)에 직접 억세스
// - let gift = Some("candy");
//   gift.unwrap() // "candy"
// - panic이 발생할 수 있다.
//
// expect: Option<T>의 내부 값(T)에 직접 억세스, 에러 메시지를 지정할 수 있다.
// - let nothing: Option<&str> = None;
//   nothing.expect("This will crash");
//
// unwrap_or
// - None.unwrap_or("cat")
//
//
// Result<T, E>: 오류를 처리하는 열거형
// enum Result<T, E> {
//    Ok(T),
//    Err(E),
// }
//
// - Node이 존재할 가능성이 있는 Option<T>형식과 달리, 
//   Error가 발생할 가능성이 있는 Result<T, E>형식
// - unwrap / expect method 제공
//
fn result_enum() {

    let d = safe_division(9.0, 3.0);
    let d = safe_division(9.0, 0.0);

    let d = match d {
        Ok(v) => v,
        Err(error) => panic!("나누기 실패 {:?}", error),
    };
}

#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

// Ownership
// - Move ownership
//   let mascot = String::from("ferris");
//   let ferris = mascot; // move ownership to ferris
//
fn ownership() {
    move_ownership();
    clone_ownership();
    copy_ownership();
    borrow_ownership();

    let mut greeting = String::from("hello");
    change(&mut greeting);

    mutable_reference();
}

// - Move ownership
fn move_ownership() {
    let s = String::from("Hello, world!");
    move_process(s); // Ownership of the string in `s` moved into `process`
    move_process(s); // Error! ownership already moved.
}

// - Clone ownership
fn clone_ownership() {
    let s = String::from("Hello, world!");
    move_process(s.clone());
    move_process(s);
}
fn move_process(input: String) {}

// - Copy ownership
fn copy_ownership() {
    let s = 1u32;
    copy_process(s); // Ownership of the string in `s` moved into `process`
    copy_process(s); // Error! ownership already moved.
}
fn copy_process(input: u32) {}

// - Borrow ownership( pass reference by using & )
fn borrow_ownership() {
    let greeting = String::from("hello");
    let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
    println!("Greeting: {}", greeting);

    let s = String::from("Hello, world!");
    borrow_process(&s); // Ownership of the string in `s` moved into `process`
    borrow_process(&s); // Error! ownership already moved.
}
fn borrow_process(input: &String) {}

// &:    reference not change value: immutable reference
// &mut: reference change value --> Only one mutable reference to a particular piece of data in a particular scope
fn change(text: &mut String) {
    text.push_str(", world");
}

// &<T> : reference
// &mut <T>: mutable reference
fn mutable_reference() {
    // multiple immutable references
    let iref = String::from("Immutable Ref");
    let iref1 = &iref;
    let iref2 = &iref; // error: cannot borrow `s` as mutable more than once at a time
    println!("{}, {}", iref1, iref2);
    //
    // Only one mutable reference
    let mut mref = String::from("Mutable Ref");
    let mref1 = &mut mref;
    let mref2 = &mut mref; // error: cannot borrow `s` as mutable more than once at a time
    // Obly one mutavle reference
    println!("{}, {}", mref1, mref2);
}

// Lifetime : 'a
// 모든 참조는 lifetime을 가지고 있다.
// y개 대여된 후 삭제되었지만, x는 여전히 참조한다 <-- error.
fn lifetime() {
    let x;
    {
        let y = 42;
        x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
    }
    println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
}
