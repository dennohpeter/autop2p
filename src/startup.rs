//! routes/startup.rs

use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

use crate::routes::{buy, health_check, sell};

pub fn run(listener: TcpListener, connection: mongodb::Client) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/buy", web::post().to(buy))
            .route("/sell", web::post().to(sell))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
