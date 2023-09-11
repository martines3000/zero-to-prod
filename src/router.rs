use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::*;

pub fn build_axum_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/subscriptions", post(subscribe))
}
