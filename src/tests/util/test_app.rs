use std::sync::Arc;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{
    app::ServerState,
    build_handler,
    configuration::{DatabaseSettings, ServerSettings},
};

pub struct TestApp {
    pub server_state: Arc<ServerState>,
    pub app: axum::Router,
}

impl TestApp {
    pub async fn init() -> (Arc<ServerState>, axum::Router) {
        let configuration = test_config();
        configure_database(&configuration.database).await;

        build_app(configuration).await
    }
}

fn test_config() -> ServerSettings {
    ServerSettings {
        application_port: 0,
        database: DatabaseSettings {
            username: "postgres".to_string(),
            password: "password".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            database_name: Uuid::new_v4().to_string(),
        },
    }
}

async fn configure_database(config: &DatabaseSettings) {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
}

async fn build_app(config: ServerSettings) -> (Arc<ServerState>, axum::Router) {
    let server_state = Arc::new(ServerState::new(config).await);

    let app = build_handler(server_state.clone());

    (server_state, app)
}
