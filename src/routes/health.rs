use axum::response::IntoResponse;

pub async fn health() -> impl IntoResponse {
    // Return a 200 OK response with no body
    ""
}
