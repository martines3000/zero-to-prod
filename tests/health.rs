use axum::http::{Request, StatusCode};
use hyper::Body;
use tower::ServiceExt;
use zero2prod::build_handler;

#[tokio::test]
async fn health_check_works() {
    let app = build_handler();

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Status code should be 200 OK and body should be empty.
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        hyper::body::to_bytes(response.into_body())
            .await
            .unwrap()
            .len(),
        0
    );
}
