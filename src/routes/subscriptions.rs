use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct SubscribeFormData {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct Subscription {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub subscribed_at: chrono::DateTime<Utc>,
}

#[axum::debug_handler]
pub async fn subscriptions(
    pool: State<PgPool>,
    form: axum::extract::Form<SubscribeFormData>,
) -> Result<impl IntoResponse, crate::error::AppError> {
    // 422 if the email or name are empty
    if form.email.is_empty() || form.name.is_empty() {
        return Err(crate::error::AppError::UnprocessableEntity(
            anyhow::anyhow!("email and name must be non-empty"),
        ));
    }

    // write the email and name to subscriptions table
    let subscribe_result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.name,
        form.email,
        Utc::now(),
    )
    .fetch_one(&*pool)
    .await;

    // 200 if everything is ok
    match subscribe_result {
        Ok(_) => Ok(Response::new("".to_string())),
        Err(_) => Err(crate::error::AppError::InternalServerError(
            anyhow::anyhow!("Database failed to execute query"),
        )),
    }
}
