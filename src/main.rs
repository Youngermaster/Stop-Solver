mod api;

fn main() {
    println!("Hello, world!");
    api::requests::make_request();
    api::requests::random_name_request();
}
