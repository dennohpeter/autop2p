//! src/models/otp.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Otp {
    pub phone_number: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtpVerification {
    pub phone_number: String,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OtpResponse {
    pub sid: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OtpVerificationResponse {
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse {
    pub status: u16,
    pub message: String,
    pub data: String,
}
