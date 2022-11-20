//! main.rs

use autop2p::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    run(listener)?.await
}
