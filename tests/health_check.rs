//! health_check.rs

use std::sync::atomic::AtomicU16;

const LOCALHOST: &str = "127.0.0.1";

static PORT: AtomicU16 = AtomicU16::new(8000);

async fn spawn_app() -> Result<String, anyhow::Error> {
    println!("Getting config...");
    let config = zero2prod::configuration::get_config()?;
    println!("Connecting to Postgres...");
    let pool = sqlx::PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let server = loop {
        let port = PORT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        println!("Attempting to bind to port {}.", port);
        match zero2prod::run(LOCALHOST.to_string(), port, pool.clone()).await {
            Ok(server) => break server,
            Err(err) => {
                println!("Failed to bind to port {}.", port);
                println!("Error: {}", err);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            }
        }
    };

    let _ = tokio::spawn(server);

    Ok(format!(
        "http://{}:{}",
        LOCALHOST,
        PORT.load(std::sync::atomic::Ordering::SeqCst)
    ))
}

#[tokio::test]
async fn health_check_works() -> Result<(), anyhow::Error> {
    let addr = spawn_app().await?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length());

    Ok(())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() -> Result<(), anyhow::Error> {
    let addr = spawn_app().await?;
    let config = zero2prod::configuration::get_config()?;
    let pool = sqlx::PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", addr))
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

    let saved = sqlx::query!("SELECT name, email FROM subscriptions",)
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");

    Ok(())
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() -> Result<(), anyhow::Error> {
    let addr = spawn_app().await?;

    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not return a 422 Unprocessable Entity when the payload was {}.",
            error_message
        );
    }

    Ok(())
}
