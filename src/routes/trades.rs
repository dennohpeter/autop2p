//! routes/trades.rs

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BuyParams {
    token: String,
    amount: u32,
    from: String,
    to: String,
}
pub async fn buy(_form: web::Form<BuyParams>) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn sell() -> impl Responder {
    HttpResponse::Ok()
}
