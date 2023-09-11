mod router;
pub mod routes;

use crate::router::build_axum_router;

pub fn build_handler() -> axum::Router {
    let axum_router = build_axum_router();

    axum_router
}
