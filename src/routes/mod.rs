//! src/lib.rs

mod health;
mod trades;

pub use health::health_check;
pub use trades::{buy, sell};
