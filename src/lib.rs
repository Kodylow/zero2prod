mod error;

use axum::{
    response::{IntoResponse, Response},
    routing::{get, post, IntoMakeService},
    Router,
};
use hyper::{server::conn::AddrIncoming, Server};
use serde::Deserialize;

pub fn run() -> Result<Server<AddrIncoming, IntoMakeService<Router>>, anyhow::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions));

    let addr = "127.0.0.1:8000".parse().unwrap();
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    Ok(server)
}

#[axum::debug_handler]
async fn health_check() -> impl IntoResponse {
    Response::new("OK".to_string())
}

#[derive(Deserialize, Debug)]
struct SubscribeFormData {
    email: String,
    name: String,
}

#[axum::debug_handler]
async fn subscriptions(
    form: axum::extract::Form<SubscribeFormData>,
) -> Result<impl IntoResponse, crate::error::AppError> {
    // 400 if the email or name are empty
    if form.email.is_empty() || form.name.is_empty() {
        return Err(crate::error::AppError(anyhow::anyhow!(
            "Empty email or name"
        )));
    }

    // 200 if everything is ok
    Ok(Response::new("OK".to_string()))
}
