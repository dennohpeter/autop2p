//! src/models/trades.rs
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Transaction {
    pub token: String,
    pub amount: rust_decimal::Decimal,
    pub from: String,
    pub to: String,
    pub fee: rust_decimal::Decimal,
}
