use reqwest;

pub fn make_request() {
    println!("This is a request");
}

pub async fn random_name_request() -> (Result<(), reqwest::Error>, String) {
    println!("Stuff");
    let res = reqwest::get("https://random-names-api.herokuapp.com/random")
        .await
        .unwrap();

    println!("Status: {}", res.status());

    let body = res.text().await.unwrap();

    println!("Body:\n\n{}", body);

    (Ok(()), body)
}
