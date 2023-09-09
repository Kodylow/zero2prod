use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = zero2prod::configuration::get_config()?;
    let pool = sqlx::PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let server = run(config.application.host, config.application.port, pool).await?;

    server.await?;

    Ok(())
}
