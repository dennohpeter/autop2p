//! routes/health.rs

use actix_web::{get, HttpResponse};

#[get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}
