use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()?;
    let _ = run(port)?.await;

    Ok(())
}
