//! health_check.rs

const LOCALHOST_ADDRESS: &str = "http://localhost:8000";

fn spawn_app() -> Result<(), anyhow::Error> {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);

    Ok(())
}

#[tokio::test]
async fn health_check_works() -> Result<(), anyhow::Error> {
    let _ = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", LOCALHOST_ADDRESS))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length());

    Ok(())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() -> Result<(), anyhow::Error> {
    let _ = spawn_app();

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", LOCALHOST_ADDRESS))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not return a 200 OK in response to a valid POST request."
    );

    Ok(())
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let _ = spawn_app();

    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", LOCALHOST_ADDRESS))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 OK when the payload was {}.",
            error_message
        );
    }
}
