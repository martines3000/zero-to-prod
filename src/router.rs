use axum::{
    routing::{get, post},
    Router,
};

use crate::{app::AppState, routes::*};

pub fn build_axum_router(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/subscriptions", post(subscribe))
        .with_state(app_state)
}
