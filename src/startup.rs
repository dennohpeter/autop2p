//! routes/startup.rs

use actix_web::{dev::Server, middleware, web, App, HttpServer};
use std::net::TcpListener;

use crate::handlers::{buy, health_check, sell, send_otp, verify_otp};

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(health_check)
            .service(send_otp)
            .service(verify_otp)
            .route("/buy", web::post().to(buy))
            .route("/sell", web::post().to(sell)),
    );
}
pub fn run(listener: TcpListener, client: mongodb::Client) -> Result<Server, std::io::Error> {
    let client = web::Data::new(client);
    let server = HttpServer::new(move || {
        let logger = middleware::Logger::default();
        App::new()
            .wrap(logger)
            .app_data(client.clone())
            .configure(config)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
