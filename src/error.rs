// impl IntoResponse for anyhow::Error
pub use axum::response::{IntoResponse, Response};
pub use hyper::StatusCode;

// Make our own error that wraps `anyhow::Error`.
pub enum AppError {
    BadRequest(anyhow::Error),
    Unauthorized(anyhow::Error),
    UnprocessableEntity(anyhow::Error),
    NotFound(anyhow::Error),
    InternalServerError(anyhow::Error),
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(err) => {
                (StatusCode::BAD_REQUEST, format!("Bad request: {}", err)).into_response()
            }
            AppError::Unauthorized(err) => {
                (StatusCode::UNAUTHORIZED, format!("Unauthorized: {}", err)).into_response()
            }
            AppError::NotFound(err) => {
                (StatusCode::NOT_FOUND, format!("Not found: {}", err)).into_response()
            }
            AppError::UnprocessableEntity(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Unprocessable entity: {}", err),
            )
                .into_response(),
            AppError::InternalServerError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )
                .into_response(),
            // Handle other cases as needed...
        }
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        AppError::InternalServerError(err.into())
    }
}
