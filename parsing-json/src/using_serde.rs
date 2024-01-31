use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    message: String,
}

fn to_and_from_json() {

    let json = json!({"message": "Hello, World!"});
    let my_struct: MyStruct = serde_json::from_str(&json).unwrap();
    println!("Struct: {:?}", my_struct);

    assert_eq!(my_struct.message, "Hello, World!".to_string());
    assert!(serde_json::to_string(&my_struct).is_ok());
}
