//! main.rs

use autop2p::startup::run;
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let uri =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to database");

    run(listener, client)?.await
}
