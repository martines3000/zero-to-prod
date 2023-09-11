use std::ops::Deref;
use std::sync::Arc;

use axum::extract::{FromRequestParts, State};
use sqlx::postgres::PgPoolOptions;

use crate::configuration;

pub struct AppState {
    // The server configuration
    pub config: configuration::ServerSettings,
    // The database connection pool
    pub pool: sqlx::PgPool,
}

impl AppState {
    pub async fn new(config: configuration::ServerSettings) -> Self {
        // Set up connection pool or panic
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect(&config.database.connection_string())
            .await
            .expect("can't connect to database");

        Self { config, pool }
    }
}

#[derive(Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct ApplicationState(pub Arc<AppState>);

// Deref so you can still access the inner fields easily
impl Deref for ApplicationState {
    type Target = AppState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
