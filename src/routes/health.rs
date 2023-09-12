use axum::response::IntoResponse;
use hyper::StatusCode;

pub async fn health() -> impl IntoResponse {
    // Return a 200 OK response with no body
    (StatusCode::OK, ())
}
