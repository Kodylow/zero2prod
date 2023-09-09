//! src/lib.rs
pub mod configuration;
pub mod error;
pub mod routes;
pub mod startup;

use std::net::{SocketAddr, TcpListener};

use axum::{
    routing::{get, post, IntoMakeService},
    Router,
};
use hyper::{server::conn::AddrIncoming, Server};

pub async fn run(
    host: String,
    port: u16,
    pool: sqlx::PgPool,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, anyhow::Error> {
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscriptions))
        .with_state(pool);
    let addr = TcpListener::bind((host, port))?.local_addr()?;
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    Ok(server)
}
