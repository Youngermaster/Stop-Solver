mod api;
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    api::requests::make_request();
    let (_, x): (_, String) = block_on(api::requests::random_name_request());
    api::data_processing::get_random_name(&x);
}
