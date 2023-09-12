use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use zero2prod::{app::ServerState, configuration::get_configuration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "zero2prod=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("Listening on {}", address);

    // Create app state
    let server_state = Arc::new(ServerState::new(configuration).await);

    let axum_router =
        zero2prod::build_handler(server_state.clone()).layer(TraceLayer::new_for_http());

    // Run with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], server_state.config.application_port));
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
    use zero2prod::{app::ServerState, build_handler, configuration::get_configuration}; // for `oneshot` and `ready`

    #[tokio::test]
    async fn test_multiple_requests() {
        let configuration = get_configuration().expect("Failed to read configuration.");
        let server_state = Arc::new(ServerState::new(configuration).await);

        let mut app = build_handler(server_state.clone());

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
