use axum::response::IntoResponse;
use chrono::Utc;
use hyper::StatusCode;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{app::AppState, validation::ValidatedForm};

#[derive(Debug, Deserialize, Validate)]
pub struct SubscribeFormInput {
    #[validate(required, length(min = 1, max = 32))]
    pub name: Option<String>,
    #[validate(required, email)]
    pub email: Option<String>,
}

pub async fn subscribe(
    state: AppState,
    ValidatedForm(data): ValidatedForm<SubscribeFormInput>,
) -> impl IntoResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        data.email,
        data.name,
        Utc::now()
    )
    .execute(&state.pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
