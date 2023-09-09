use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let _ = run()?.await;

    Ok(())
}
