use std::{net::SocketAddr, sync::Arc};
use zero2prod::{app::AppState, configuration::get_configuration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("Listening on {}", address);

    // Create app state
    let app_state = Arc::new(AppState::new(configuration).await);

    let axum_router = zero2prod::build_handler(app_state.clone());

    // Run with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], app_state.config.application_port));
    axum::Server::bind(&addr)
        .serve(axum_router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::Service; // for `call`
    use tower::ServiceExt;
    use zero2prod::{app::AppState, build_handler, configuration::get_configuration}; // for `oneshot` and `ready`

    #[tokio::test]
    async fn test_multiple_requests() {
        let configuration = get_configuration().expect("Failed to read configuration.");
        let app_state = Arc::new(AppState::new(configuration).await);

        let mut app = build_handler(app_state.clone());

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();
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
