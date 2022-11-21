//! src/handlers/otp.rs

use actix_web::{post, web, HttpResponse};
use reqwest::StatusCode;

use crate::models::{ApiResponse, Otp, OtpVerification};
use crate::services::TwilioService;

#[post("/otp")]
pub async fn send_otp(data: web::Json<Otp>) -> HttpResponse {
    let data = Otp {
        phone_number: data.phone_number.clone(),
    };

    let res = TwilioService::send_otp(&data.phone_number).await;

    match res {
        Ok(otp) => HttpResponse::Ok().json(ApiResponse {
            status: StatusCode::ACCEPTED.as_u16(),
            data: otp.sid,
            message: "success".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failed".to_string(),
            data: e.to_string(),
        }),
    }
}

#[post("/otp/verify")]
pub async fn verify_otp(data: web::Json<OtpVerification>) -> HttpResponse {
    let data = OtpVerification {
        phone_number: data.phone_number.clone(),
        code: data.code.clone(),
    };

    let res = TwilioService::verify_otp(&data.phone_number, &data.code).await;

    match res {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            status: StatusCode::ACCEPTED.as_u16(),
            data: "OTP verified successfully".to_string(),
            message: "success".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failed".to_string(),
            data: e.to_string(),
        }),
    }
}
