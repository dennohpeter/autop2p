//! src/services/twilio.rs

use std::{collections::HashMap, env};

use dotenv::dotenv;
use reqwest::{header, Client};

use crate::models::{OtpResponse, OtpVerificationResponse};

pub struct TwilioService {}

impl TwilioService {
    fn env_loader(key: &str) -> String {
        dotenv().ok();
        match env::var(key) {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        }
    }

    pub async fn send_otp(phone_number: &String) -> Result<OtpResponse, &'static str> {
        let account_sid = TwilioService::env_loader("TWILIO_ACCOUNT_SID");
        let auth_token = TwilioService::env_loader("TWILIO_AUTH_TOKEN");
        let service_id = TwilioService::env_loader("TWILIO_SERVICE_SID");

        let url = format!(
            "https://verify.twilio.com/v2/Services/{}/Verifications",
            service_id
        );

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut form_body: HashMap<&str, String> = HashMap::new();
        form_body.insert("To", phone_number.to_string());
        form_body.insert("Channel", "sms".to_string());

        let client = Client::new();
        let res = client
            .post(url)
            .basic_auth(account_sid, Some(auth_token))
            .headers(headers)
            .form(&form_body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let result = response.json::<OtpResponse>().await;
                match result {
                    Ok(data) => Ok(data),
                    Err(_) => Err("Error sending OTP"),
                }
            }
            Err(_) => Err("Error sending OTP"),
        }
    }

    pub async fn verify_otp(phone_number: &String, code: &String) -> Result<(), &'static str> {
        let account_sid = TwilioService::env_loader("TWILIO_ACCOUNT_SID");
        let auth_token = TwilioService::env_loader("TWILIO_AUTH_TOKEN");
        let service_id = TwilioService::env_loader("TWILIO_SERVICE_SID");

        let url = format!(
            "https://verify.twilio.com/v2/Services/{}/VerificationCheck",
            service_id,
        );

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut form_body: HashMap<&str, &String> = HashMap::new();
        form_body.insert("To", phone_number);
        form_body.insert("Code", code);

        let client = Client::new();
        let res = client
            .post(url)
            .basic_auth(account_sid, Some(auth_token))
            .headers(headers)
            .form(&form_body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let data = response.json::<OtpVerificationResponse>().await;
                match data {
                    Ok(result) => {
                        if result.status == "approved" {
                            Ok(())
                        } else {
                            Err("Error verifying OTP")
                        }
                    }
                    Err(_) => Err("Error verifying OTP"),
                }
            }
            Err(_) => Err("Error verifying OTP"),
        }
    }
}
