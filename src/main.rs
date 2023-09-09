use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // Run with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
}

fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
}

async fn index() -> impl IntoResponse {
    Json(json!({
        "message": "Hello, World!",
    }))
}

async fn health_check() -> impl IntoResponse {
    // Return a 200 OK response with no body
    ""
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::{json, Value};
    use tower::Service; // for `call`
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn test_index() {
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "message": "Hello, World!" }));
    }

    #[tokio::test]
    async fn test_multiple_requests() {
        let mut app = app();

        let request = Request::builder().uri("/").body(Body::empty()).unwrap();
        let response = app.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();
        let response = app.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
