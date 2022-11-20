//! tests/health.rs

use std::net::TcpListener;

/// Spin up an instance of our application
/// and returns its address as a string
/// i.e. http://127.0.0.1:XXXX
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let server = autop2p::startup::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health", &address))
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
    let address = spawn_app();

    let client = reqwest::Client::new();

    let body = "token=abc&amount=100&from=%2B2547&to=0x456";

    // Act
    let response = client
        .post(&format!("{}/buy", &address))
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
    let address = spawn_app();

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
            .post(&format!("{}/buy", &address))
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
