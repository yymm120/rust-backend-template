use sqlx::PgPool;
use axum::{extract::State, routing::get, Router};


pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

// #[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn version(State(state): State<std::sync::Arc<AppState>>) -> String {
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(&state.pool.clone())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}