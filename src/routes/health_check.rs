use axum::response::{IntoResponse, Response};

#[axum::debug_handler]
pub async fn health_check() -> impl IntoResponse {
    Response::new("OK".to_string())
}
