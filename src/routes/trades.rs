//! routes/trades.rs

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Transaction {
    token: String,
    amount: rust_decimal::Decimal,
    from: String,
    to: String,
    fee: rust_decimal::Decimal,
}

pub async fn buy(_payload: web::Json<Transaction>) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn sell() -> impl Responder {
    HttpResponse::Ok()
}
