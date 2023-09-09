use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    run().await
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::Service; // for `call`
    use tower::ServiceExt;
    use zero2prod::app; // for `oneshot` and `ready`

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
