use axum::{routing::get, Router};
use sqlx::Executor;
use shuttle_runtime::CustomError;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
    // #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
