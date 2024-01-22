use serde_json::{Result, Value};

fn main() {
    println!("Parsing JSON using serde!");
    untyped_example().unwrap();
}

fn untyped_example() -> Result<()> {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the steing of fdata into serde_json::Value
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square bracket's
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
