fn main() {
    let mut a_num: i32 = 0;
    let a_word = "Ten";

    a_num = 10;

    println!("The Number is {}", a_num);
    println!("The word is {}", a_word);

    shadow_variable();
}

// shadow: mutable 선언 없이 변수를 변경
fn shadow_variable() {
    let shadow_num = 5;
    let shadow_num = shadow_num + 1;
    let shadow_num = shadow_num * 2;

    // shadow_num = 10; // error: cannot assign twice to immutable variable

    println!("The value of shadow_num is: {}", shadow_num);

    string_type();
}


fn string_type() {
    let character_1: char = 's';
    let character_2: char = 'f';

    let smiley_face = '😃';
    let string_1 = "miley ";

    let string_2: &str = "face";
    // let string_2: str = "face";

    println!("{} is {}{}{}{}.", smiley_face, character_1, character_2, string_1, string_2);
}
