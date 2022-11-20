//! tests/health.rs

use std::net::TcpListener;

use mongodb::Client;

pub struct TestApp {
    pub address: String,
    pub db: mongodb::Database,
}
/// Spin up an instance of our application
/// and returns TestApp wrapper around it
/// i.e. http://127.0.0.1:XXXX
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let uri =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to database");
    let server = autop2p::startup::run(listener, client.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db: client.database("autop2p"),
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length());
}

#[tokio::test]
async fn buy_returns_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let body = "token=abc&amount=100&from=%2B2547&to=0x456";

    // Act
    let response = client
        .post(&format!("{}/buy", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn buy_returns_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let test_cases = vec![
        (
            "abc&amount=100&from=%2B2547&to=0x456",
            "Token address is required",
        ),
        ("token=abc&from=%2B2547&to=0x456", "Amount is required"),
        (
            "token=abc&amount=100&to=0x456",
            "From phone number is required",
        ),
        (
            "token=abc&amount=100&from=%2B2547",
            "To address is required",
        ),
    ];

    for (body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/buy", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(400, response.status().as_u16());
        // assert_eq!(Some(error_message.len() as u64), response.content_length());
        assert_eq!(Some(error_message.to_string()), response.text().await.ok());
    }
}
