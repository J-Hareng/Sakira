use std::env::var;

pub fn init() {
    let _ = dotenv::dotenv().unwrap();
}
