use axum::{extract::State, routing::get, Router};
use shuttle_runtime::CustomError;
use sqlx::Executor;


use api_lib::health::*;

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
