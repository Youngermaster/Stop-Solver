use reqwest;
// use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};

pub fn make_request() {
    println!("This is a request");
}

#[tokio::main]
pub async fn random_name_request() -> Result<(), reqwest::Error> {
    println!("Stuff");
    let res = reqwest::get("https://random-names-api.herokuapp.com/random").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}

// pub fn get_random_name(body: &str) -> Result<()> {
//     // Parse the string of data into serde_json::Value.
//     let v: Value = serde_json::from_str(body)?;

//     // Access parts of the data by indexing with square brackets.
//     // println!("Please call {} at the number {}", v["name"], v["phones"][0]);
//     println!("Body {}", v["body"]);

//     Ok(())
// }
