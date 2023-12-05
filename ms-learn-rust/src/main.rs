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
}

// shadow: mutable 선언 없이 변수를 변경, 
// - not mutable
fn shadow_variable() {
    let shadow_num = 5;
    let shadow_num = shadow_num + 1;
    let shadow_num = shadow_num * 2;

    // shadow_num = 10; // error: cannot assign twice to immutable variable

    println!("The value of shadow_num is: {}", shadow_num);
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

    println!("{} is {}{}{}{}.", smiley_face, character_1, character_2, string_1, string_2);
}

// Arrays
// - [T; N] T: type, N: length, size는 고정
fn arrays() {
    let array_1 = [1, 2, 3];
    let array_2: [i32; 3] = [1, 2, 3];
    let array_3 = [3; 3]; // [3, 3, 3]

    println!("The first element  of array_1 is: {}", array_1[0]);
    println!("The second element of array_2 is: {}", array_2[1]);
    println!("The third element  of array_3 is: {}", array_3[2]);

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
    let vector_5 = vector_1[5];
    println!("The sixth element of vector_1 is: {:?}", vector_1[5]); // error: index out of bounds
    //
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
fn hash_map() {

}
