//! src/routes/mod.rs

mod health;
mod otp;
mod trades;

pub use health::health_check;
pub use otp::{send_otp, verify_otp};
pub use trades::{buy, sell};
