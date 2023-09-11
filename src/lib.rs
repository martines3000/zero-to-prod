pub mod app;
pub mod configuration;
mod router;
pub mod routes;
pub mod validation;

use std::sync::Arc;

use app::{AppState, ApplicationState};

use crate::router::build_axum_router;

pub fn build_handler(app_state: Arc<AppState>) -> axum::Router {
    let state = ApplicationState(app_state);

    

    build_axum_router(state)
}
