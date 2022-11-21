//! routes/trades.rs

use crate::models::Transaction;
use actix_web::{web, HttpResponse, Responder};

pub async fn buy(_payload: web::Json<Transaction>) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn sell() -> impl Responder {
    HttpResponse::Ok()
}
