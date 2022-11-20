//! routes/startup.rs

use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

use crate::routes::{buy, health_check, sell};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/buy", web::post().to(buy))
            .route("/sell", web::post().to(sell))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
