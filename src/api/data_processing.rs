use serde_json::{Result, Value};

pub fn get_random_name(body: &str) -> Result<()> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(body)?;
    // Access parts of the data by indexing with square brackets.
    println!("Name: {}", v["body"]["name"]);
    Ok(())
}
