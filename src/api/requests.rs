use reqwest;

pub fn echo_request(echo: &str) {
    println!("{}", echo);
}

pub async fn random_name_request() -> (Result<(), reqwest::Error>, String) {
    let res = reqwest::get("https://random-names-api.herokuapp.com/random")
        .await
        .unwrap();

    let body = res.text().await.unwrap();

    (Ok(()), body)
}
