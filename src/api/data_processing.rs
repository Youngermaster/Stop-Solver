use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

pub fn get_random_name(body: &str) -> Result<()> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(body)?;
    // Access parts of the data by indexing with square brackets.
    // println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    println!("Name: {}", v["body"]["name"]);
    Ok(())
}
