use std::env;

mod cfg;
mod server;

#[tokio::main]
async fn main() {
    cfg::init();

    let server = server::server::Server::new(
        dotenv::var("HOST").unwrap_or("127.0.0.1".to_string()),
        dotenv::var("PORT")
            .unwrap_or("8080".to_string())
            .parse::<u16>()
            .unwrap(),
    );

    server.run().await;
}
