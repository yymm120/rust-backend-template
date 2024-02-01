use axum::{extract::State, routing::get, Router};
use shuttle_runtime::CustomError;
use sqlx::Executor;
use sqlx::PgPool;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

// #[derive(Clone)]
struct AppState {
    pool: PgPool,
}

async fn version(State(state): State<std::sync::Arc<AppState>>) -> String {
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(&state.pool.clone())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
    // #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = std::sync::Arc::new(AppState { pool });
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/version", get(version))
        .with_state(state);

    Ok(router.into())
}
