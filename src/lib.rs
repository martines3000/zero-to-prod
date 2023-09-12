pub mod app;
pub mod configuration;
mod router;
pub mod routes;
pub mod validation;

use std::sync::Arc;

use app::{AppState, ServerState};

use crate::router::build_axum_router;

pub fn build_handler(server_state: Arc<ServerState>) -> axum::Router {
    let state = AppState(server_state);

    build_axum_router(state)
}
