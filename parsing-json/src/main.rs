use serde_json::{Result, Value};
use serde_json::json;
use serde::{Deserialize, Serialize}; // install with features = ["derive"] for using derive macros

// Parsing JSON using serde!
#[derive(Serialize, Deserialize, Debug)]
pub struct MyStruct {
    message: String,
}

fn main() {
    println!("Parsing JSON using serde!");
    untyped_example().unwrap();
    convert_json_to_struct();
}

fn untyped_example() -> Result<()> {
    let json_string = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the steing of fdata into serde_json::Value
    let v: Value = serde_json::from_str(json_string)?;

    // Access parts of the data by indexing with square bracket's
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

fn convert_json_to_struct() {
    let literal_json_string = r#"
        {
            "message": "Hello, World!"
        }"#;
    let raw_json_string = &json!({ "message": "Hello, World!" }).to_string();
    // let raw_json_string = json!({ "message": "Hello, World!" });

    let my_struct:  MyStruct = serde_json::from_str(literal_json_string).unwrap();
    let my_struct2: MyStruct = serde_json::from_str(raw_json_string).unwrap();
    println!("My struct: {:?}", my_struct);
    println!("My struct2: {:?}", my_struct2);
}
