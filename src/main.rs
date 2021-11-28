mod api;
mod cli;
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    cli::terminal_interface::start();
    api::requests::echo_request("Initializing API...");
    let (_, x): (_, String) = block_on(api::requests::random_name_request());
    api::data_processing::get_random_name(&x);
}
